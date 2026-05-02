import { z } from 'zod';

const password = z.string().min(8, 'Password must be at least 8 characters');
const email = z.string().email('Invalid email address');

export const loginSchema = z.object({
	nameOrEmail: z.string().min(1, 'Name or email is required'),
	password: z.string().min(1, 'Password is required')
});

export const registerSchema = z
	.object({
		name: z.string().min(1, 'Name is required'),
		email,
		password,
		confirmPassword: z.string().min(1, 'Please confirm your password')
	})
	.refine((d) => d.password === d.confirmPassword, {
		message: 'Passwords do not match',
		path: ['confirmPassword']
	});

export const forgotPasswordSchema = z.object({ email });

export const resetPasswordSchema = z
	.object({
		token: z.string().min(1, 'Token is required'),
		newPassword: password,
		confirmPassword: z.string().min(1, 'Please confirm your password')
	})
	.refine((d) => d.newPassword === d.confirmPassword, {
		message: 'Passwords do not match',
		path: ['confirmPassword']
	});

export const resendVerificationSchema = z.object({ email });

export const changePasswordSchema = z
	.object({
		currentPassword: z.string().min(1, 'Current password is required'),
		newPassword: password,
		confirmPassword: z.string().min(1, 'Please confirm your password')
	})
	.refine((d) => d.newPassword === d.confirmPassword, {
		message: 'Passwords do not match',
		path: ['confirmPassword']
	});

export type LoginInput = z.infer<typeof loginSchema>;
export type RegisterInput = z.infer<typeof registerSchema>;
export type ForgotPasswordInput = z.infer<typeof forgotPasswordSchema>;
export type ResetPasswordInput = z.infer<typeof resetPasswordSchema>;
export type ResendVerificationInput = z.infer<typeof resendVerificationSchema>;
export type ChangePasswordInput = z.infer<typeof changePasswordSchema>;
