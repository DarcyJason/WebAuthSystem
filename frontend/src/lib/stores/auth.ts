import { writable, get } from 'svelte/store';

export const accessToken = writable<string | null>(null);

export function logout() {
    accessToken.set(null);
}

export async function authenticatedFetch(input: RequestInfo | URL, init?: RequestInit): Promise<Response> {
    const token = get(accessToken);
    if (token) {
        const headers = new Headers(init?.headers);
        if (!headers.has('Authorization')) {
            headers.set('Authorization', `Bearer ${token}`);
        }
        return fetch(input, { ...init, headers });
    }
    return fetch(input, init);
}
