import type { User } from '.';

export interface AuthResponse {
	status: 'success' | 'error';
	message: string;
}

export interface RegisterRequest {
	name: string;
	email: string;
	password: string;
	confirm_password: string;
}

export interface RegisterResponse {
	status: 'success' | 'error';
	message: string;
	data?: {
		user: User;
		verification_sent: boolean;
	};
}

export interface LoginRequest {
	email: string;
	password: string;
}

export interface LoginResponse {
	status: 'success' | 'error';
	message: string;
	data?: {
		access_token: string;
		refresh_token: string;
		user: User;
	};
}

export interface ForgotPasswordRequest {
	email: string;
}

export interface ResetPasswordRequest {
	token: string;
	password: string;
	password_confirm: string;
}

export interface ResetPasswordResponse {
	status: 'success' | 'error';
	message: string;
}

export interface VerifyEmailRequest {
	token: string;
}

export interface VerifyEmailResponse {
	status: 'success' | 'error';
	message: string;
	verified: boolean;
}
