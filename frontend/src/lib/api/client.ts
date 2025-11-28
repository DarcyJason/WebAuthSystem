import { API_CONFIG } from "$lib/constants";
import type { ApiError } from "$lib/types";

export interface RequestConfig {
    headers?: Record<string, string>;
    params?: Record<string, string | number | boolean>;
    timeout?: number;
    retry?: number;
    data?: any;
}

export interface RequestInterceptor {
    (config: RequestConfig): RequestConfig | Promise<RequestConfig>;
}

export interface ResponseInterceptor {
    (response: Response): Response | Promise<Response>;
}

export interface ResponseErrorInterceptor {
    (error: Error): Error | Promise<Error>;
}

export class ApiClientError extends Error {
    public code: string;

    constructor(message: string, error: ApiError) {
        super(message);
        this.name = "ApiClientError";
        this.code = error.code;
    }
}

export class ApiClient {
    private baseURL: string;
    private timeout: number;
    private retryAttempts: number;
    private retryDelay: number;
    private accessToken: string | null = null;
    private refreshToken: string | null = null;
    private requestInterceptors: RequestInterceptor[] = [];
    private responseInterceptors: ResponseInterceptor[] = [];
    private responseErrorInterceptors: ResponseErrorInterceptor[] = [];

    constructor(config: Partial<typeof API_CONFIG> = {}) {
        this.baseURL = config?.BASE_URL || API_CONFIG.BASE_URL;
        this.timeout = config?.TIMEOUT || API_CONFIG.TIMEOUT;
        this.retryAttempts = config?.RETRY_ATTEMPTS || API_CONFIG.RETRY_ATTEMPTS;
        this.retryDelay = config?.RETRY_DELAY || API_CONFIG.RETRY_DELAY;
    }
    setAccessToken(token: string | null): void {
        this.accessToken = token;
    }
    getAccessToken(): string | null {
        return this.accessToken;
    }
    setRefreshToken(token: string | null): void {
        this.refreshToken = token;
    }
    getRefreshToken(): string | null {
        return this.refreshToken;
    }
    addRequestInterceptor(interceptor: RequestInterceptor): void {
        this.requestInterceptors.push(interceptor);
    }
    addResponseInterceptor(interceptor: ResponseInterceptor): void {
        this.responseInterceptors.push(interceptor);
    }
    addResponseErrorInterceptor(interceptor: ResponseErrorInterceptor): void {
        this.responseErrorInterceptors.push(interceptor);
    }

    private async executeRequestInterceptors(config: RequestConfig): Promise<RequestConfig> {
        let finalConfig = config;
        for (const interceptor of this.requestInterceptors) {
            finalConfig = await interceptor(finalConfig);
        }
        return finalConfig;
    }

    private async executeResponseInterceptors(response: Response): Promise<Response> {
        let finalResponse = response;
        for (const interceptor of this.responseInterceptors) {
            finalResponse = await interceptor(finalResponse);
        }
        return finalResponse;
    }

    private async executeResponseErrorInterceptors(error: Error): Promise<Error> {
        let finalError = error;
        for (const interceptor of this.responseErrorInterceptors) {
            finalError = await interceptor(finalError);
        }
        return finalError;
    }

    private buildURL(url: string, params?: Record<string, string | number | boolean>): string {
        const fullURL = new URL(url.startsWith('http') ? url : this.baseURL + url);
        if (params) {
            Object.entries(params).forEach(([key, value]) => {
                fullURL.searchParams.append(key, String(value));
            });
        }
        return fullURL.toString();
    }

    private buildHeaders(config: RequestConfig): Record<string, string> {
        const headers: Record<string, string> = {
            "Content-Type": "application/json",
            ...config.headers,
        }
        if (this.accessToken && !headers["Authorization"] && this.refreshToken && !headers["Cookie"]) {
            headers["Authorization"] = `Bearer ${this.accessToken}`;
            headers["Cookie"] = `refreshToken=${this.refreshToken}`;
        }
        return headers
    }

    private createTimeoutPromise(ms: number): Promise<never> {
        return new Promise((_, reject) => {
            setTimeout(() => {
                reject(new Error(`Request timeout after ${ms}ms`));
            }, ms);
        });
    }

    private async executeRequest(url: string, method: string, config: RequestConfig, attempt: number = 0): Promise<Response> {
        try {
            const finalConfig = await this.executeRequestInterceptors(config);
            const fullURL = this.buildURL(url, finalConfig.params);
            const headers = this.buildHeaders(finalConfig);
            const timeout = finalConfig.timeout || this.timeout;
            const fetchOptions: RequestInit = {
                method,
                headers,
                signal: AbortSignal.timeout(timeout),
            };
            if (finalConfig.data && method !== "GET" && method !== "HEAD") {
                fetchOptions.body = JSON.stringify(finalConfig.data);
            }
            const response = await Promise.race([
                fetch(fullURL, fetchOptions),
                this.createTimeoutPromise(timeout)
            ]);
            return await this.executeResponseInterceptors(response);
        } catch (error) {
            const err = error instanceof Error ? error : new Error(String(error));
            if (attempt < this.retryAttempts - 1) {
                const shouldRetry =
                    err.message.includes('timeout') ||
                    err.message.includes('NetworkError') ||
                    err.message.includes('Failed to fetch');
                if (shouldRetry) {
                    await new Promise((resolve) => setTimeout(resolve, this.retryDelay * (attempt + 1)));
                    return this.executeRequest(url, method, config, attempt + 1);
                }
            }
            const finalError = await this.executeResponseErrorInterceptors(err);
            throw finalError;
        }
    }
    private async handleResponse<T = any>(response: Response): Promise<T> {
        const contentType = response.headers.get("Content-Type");
        const isJson = contentType?.includes("application/json");
        let data: any;
        try {
            data = isJson ? await response.json() : await response.text();
        } catch {
            data = null;
        }
        if (!response.ok) {
            const error: ApiError = {
                code: String(response.status),
                message: data?.message || data?.error || response.statusText,
            }
            throw new ApiClientError(error.message, error);
        }
        return data as T;
    }
    async get<T = any>(url: string, config?: RequestConfig): Promise<T> {
        const response = await this.executeRequest(url, "GET", config || {});
        return this.handleResponse<T>(response);
    }
    async post<T = any>(url: string, data?: any, config?: RequestConfig): Promise<T> {
        const finalConfig = { ...config, data };
        const response = await this.executeRequest(url, "POST", finalConfig);
        return this.handleResponse<T>(response);
    }
    async put<T = any>(url: string, data?: any, config?: RequestConfig): Promise<T> {
        const finalConfig = { ...config, data };
        const response = await this.executeRequest(url, "PUT", finalConfig);
        return this.handleResponse<T>(response);
    }
    async patch<T = any>(url: string, data?: any, config?: RequestConfig): Promise<T> {
        const finalConfig = { ...config, data };
        const response = await this.executeRequest(url, "PATCH", finalConfig);
        return this.handleResponse<T>(response);
    }
    async delete<T = any>(url: string, config?: RequestConfig): Promise<T> {
        const response = await this.executeRequest(url, "DELETE", config || {});
        return this.handleResponse<T>(response);
    }
}

export const apiClient = new ApiClient();