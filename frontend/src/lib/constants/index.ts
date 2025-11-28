export const API_CONFIG = {
	BASE_URL: import.meta.env.VITE_API_BASE_URL || 'http://localhost:7878/api/v1',
	TIMEOUT: 30000,
	RETRY_ATTEMPTS: 3,
	RETRY_DELAY: 1000,
} as const;

export const API_BASE_URL = API_CONFIG.BASE_URL;

export const API_ENDPOINTS = {
	AUTH: {
		LOGIN: '/auth/login',
		REGISTER: '/auth/register',
		LOGOUT: '/auth/logout',
		REFRESH: '/auth/refresh-token',
		VERIFY_EMAIL: '/auth/verify-email',
		RESEND_VERIFICATION: '/auth/resend-verification',
		FORGOT_PASSWORD: '/auth/forgot-password',
		RESET_PASSWORD: '/auth/reset-password'
	},
	USER: {
		ME: '/user/me',
		CHANGE_PASSWORD: '/user/change-password'
	}
} as const;

export const ROUTES = {
	HOME: '/',
	LOGIN: '/login',
	REGISTER: '/register',
	DASHBOARD: '/dashboard',
	FORGOT_PASSWORD: '/forget-password',
	RESET_PASSWORD: '/reset-password',
	VERIFY_EMAIL: '/verify-email',
	NOT_FOUND: '/404',
	ERROR: '/error'
} as const;

export const STORAGE_KEYS = {
	ACCESS_TOKEN: 'access_token',
	REFRESH_TOKEN: 'refresh_token',
	USER_INFO: 'user_info'
} as const;

export const VALIDATION_RULES = {
	EMAIL_PATTERN: /^[^\s@]+@[^\s@]+\.[^\s@]+$/,
	PASSWORD_MIN_LENGTH: 8,
	PASSWORD_PATTERN: /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]/,
	USERNAME_MIN_LENGTH: 3,
	USERNAME_MAX_LENGTH: 20,
	USERNAME_PATTERN: /^[a-zA-Z0-9_-]+$/
} as const;

export const ERROR_MESSAGES = {
	NETWORK_ERROR: 'Network connect error, please try later',
	SERVER_ERROR: 'An internal server error occur, please try later',
	INVALID_CREDENTIALS: 'Email or password is wrong',
	EMAIL_ALREADY_EXISTS: 'This email already exists',
	INVALID_EMAIL: 'Invalid email',
	INVALID_PASSWORD: 'Invalid password',
	PASSWORD_MISMATCH: "Password doesn't match confirm password",
	UNAUTHORIZED: "You don't have the right to do that",
	USER_NOT_FOUND: 'User not found',
	VERIFICATION_FAILED: 'Failed to verify your email',
	INVALID_TOKEN: 'Invalid token'
} as const;

export const SUCCESS_MESSAGES = {
	LOGIN_SUCCESS: 'Login success',
	REGISTER_SUCCESS: 'Register success',
	LOGOUT_SUCCESS: 'logout success',
	PASSWORD_RESET_SUCCESS: 'Reset password success',
	EMAIL_VERIFIED_SUCCESS: 'Success to verify your email',
	VERIFICATION_SENT: 'An verification email has been sent',
	PASSWORD_CHANGED: 'Success to change your password'
} as const;

export const HTTP_METHODS = {
	GET: 'GET',
	POST: 'POST',
	PUT: 'PUT',
	DELETE: 'DELETE',
	PATCH: 'PATCH'
} as const;

export const HTTP_STATUS = {
	OK: 200,
	CREATED: 201,
	NO_CONTENT: 204,
	BAD_REQUEST: 400,
	UNAUTHORIZED: 401,
	FORBIDDEN: 403,
	NOT_FOUND: 404,
	CONFLICT: 409,
	INTERNAL_SERVER_ERROR: 500,
	SERVICE_UNAVAILABLE: 503
} as const;

export interface ApiResponse<T = any> {
	success: boolean;
	data?: T;
	error?: string;
	message?: string;
}

export const NOTIFICATION_TYPES = {
	SUCCESS: 'success',
	ERROR: 'error',
	WARNING: 'warning',
	INFO: 'info'
} as const;

export const NOTIFICATION_DURATION = {
	SHORT: 3000,
	MEDIUM: 5000,
	LONG: 7000,
	PERMANENT: 0
} as const;

export const TIMEOUTS = {
	API_REQUEST: 30000,
	TOKEN_REFRESH: 5000,
	FORM_SUBMIT: 10000
} as const;

export const ENVIRONMENT = {
	DEV: 'development',
	PROD: 'production',
	TEST: 'test'
} as const;

export const IS_DEV = import.meta.env.MODE === 'development';
export const IS_PROD = import.meta.env.MODE === 'production';
