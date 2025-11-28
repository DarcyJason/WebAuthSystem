export interface User {
	id: string;
	name: string;
	email: string;
	is_email_verified: boolean;
}

export interface ApiError {
	code: string;
	message: string;
	field?: string;
}

export interface ApiErrorResponse {
	status: 'error';
	error: ApiError;
}

export interface Notification {
	id: string;
	type: 'success' | 'error' | 'warning' | 'info';
	message: string;
	duration?: number;
	timestamp: number;
}

export interface RouteParams {
	[key: string]: string | string[];
}

export interface NavigationGuard {
	canActivate: (route: any) => boolean | Promise<boolean>;
	canDeactivate?: (route: any) => boolean | Promise<boolean>;
}

export interface ApiResponse<T = any> {
	status: 'success' | 'error';
	message: string;
	data?: T;
}
