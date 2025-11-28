import { API_ENDPOINTS } from '$lib/constants';
import type { User } from '$lib/types';
import type {
	AuthResponse,
	ForgotPasswordRequest,
	LoginRequest,
	LoginResponse,
	RegisterRequest,
	RegisterResponse,
	ResetPasswordRequest,
	ResetPasswordResponse,
	VerifyEmailRequest,
	VerifyEmailResponse
} from '$lib/types/auth';
import { apiClient, ApiClientError } from './client';

export class AuthService {
	async register(credentials: RegisterRequest): Promise<RegisterResponse> {
		try {
			const response = await apiClient.post<RegisterResponse>(
				API_ENDPOINTS.AUTH.REGISTER,
				credentials
			);
			return response;
		} catch (error) {
			throw this.handleError(error, 'Register failed');
		}
	}
	async login(request: LoginRequest): Promise<LoginResponse> {
		try {
			const response = await apiClient.post<LoginResponse>(API_ENDPOINTS.AUTH.LOGIN, request);
			if (response.data?.access_token) {
				localStorage.setItem('access_token', response.data.access_token);
				apiClient.setAccessToken(response.data.access_token);
			}
			if (response.data?.refresh_token) {
				localStorage.setItem('refresh_token', response.data.refresh_token);
			}
			if (response.data?.user) {
				localStorage.setItem('user_info', JSON.stringify(response.data.user));
			}
			return response;
		} catch (error) {
			throw this.handleError(error, 'Login failed');
		}
	}
	async logout(): Promise<void> {
		try {
			await apiClient.post(API_ENDPOINTS.AUTH.LOGOUT);
		} catch (error) {
			console.error('Logout failed: ', error);
		} finally {
			this.clearAuthData();
			apiClient.setAccessToken(null);
		}
	}
	async refreshToken(): Promise<LoginResponse> {
		try {
			const refreshToken = localStorage.getItem('refresh_token');
			if (!refreshToken) {
				throw new Error('Refresh token not found');
			}
			const response = await apiClient.post<LoginResponse>(API_ENDPOINTS.AUTH.REFRESH, {
				refresh_token: refreshToken
			});
			if (response.data?.access_token) {
				localStorage.setItem('access_token', response.data.access_token);
				apiClient.setAccessToken(response.data.access_token);
			}
			return response;
		} catch (error) {
			this.clearAuthData();
			throw this.handleError(error, 'Refresh tokens failed');
		}
	}
	async verifyEmail(request: VerifyEmailRequest): Promise<VerifyEmailResponse> {
		try {
			const response = await apiClient.post<VerifyEmailResponse>(
				API_ENDPOINTS.AUTH.VERIFY_EMAIL,
				request
			);
			return response;
		} catch (error) {
			throw this.handleError(error, 'Verify email failed');
		}
	}
	async resendVerification(email: string): Promise<AuthResponse> {
		try {
			const response = await apiClient.post<AuthResponse>(API_ENDPOINTS.AUTH.RESEND_VERIFICATION, {
				email
			});
			return response;
		} catch (error) {
			throw this.handleError(error, 'Resend an verification email failed');
		}
	}
	async forgotPassword(request: ForgotPasswordRequest): Promise<AuthResponse> {
		try {
			const response = await apiClient.post<AuthResponse>(
				API_ENDPOINTS.AUTH.FORGOT_PASSWORD,
				request
			);
			return response;
		} catch (error) {
			throw this.handleError(error, 'Send an email to reset password failed');
		}
	}
	async resetPassword(request: ResetPasswordRequest): Promise<ResetPasswordResponse> {
		try {
			const response = await apiClient.post<ResetPasswordResponse>(
				API_ENDPOINTS.AUTH.RESET_PASSWORD,
				request
			);
			return response;
		} catch (error) {
			throw this.handleError(error, 'Reset password failed');
		}
	}
	getCurrentUser(): User | null {
		if (typeof window === 'undefined') {
			return null;
		}

		const userInfo = localStorage.getItem('user_info');
		if (!userInfo) {
			return null;
		}

		try {
			return JSON.parse(userInfo) as User;
		} catch {
			return null;
		}
	}
	getAccessToken(): string | null {
		if (typeof window === 'undefined') {
			return null;
		}
		return localStorage.getItem('access_token');
	}
	isAuthenticated(): boolean {
		return this.getAccessToken() !== null && this.getCurrentUser() !== null;
	}
	private clearAuthData(): void {
		if (typeof window !== 'undefined') {
			localStorage.removeItem('access_token');
			localStorage.removeItem('refresh_token');
			localStorage.removeItem('user_info');
			localStorage.removeItem('auth_state');
		}
	}
	private handleError(error: unknown, defaultMessage: string): Error {
		if (error instanceof ApiClientError) {
			return new Error(error.message || defaultMessage);
		}
		if (error instanceof Error) {
			return new Error(error.message || defaultMessage);
		}
		return new Error(defaultMessage);
	}
}
