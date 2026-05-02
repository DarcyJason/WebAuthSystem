<script lang="ts">
	import { goto } from '$app/navigation';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import PasswordInput from '$lib/components/PasswordInput.svelte';
	import { Label } from '$lib/components/ui/label';
	import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '$lib/components/ui/card';
	import { resetPasswordSchema } from '$lib/schemas';
	import { toast } from 'svelte-sonner';

	let token = $state('');
	let newPassword = $state('');
	let confirmPassword = $state('');
	let fieldErrors = $state<Record<string, string>>({});
	let loading = $state(false);

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		fieldErrors = {};

		const result = resetPasswordSchema.safeParse({ token, newPassword, confirmPassword });
		if (!result.success) {
			fieldErrors = Object.fromEntries(result.error.issues.map((i) => [i.path[0], i.message]));
			return;
		}

		loading = true;
		try {
			const res = await fetch('/api/v1/auth/reset-password', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify(result.data)
			});

			if (res.ok) {
				toast.success('Password reset successfully. Please sign in.');
				goto('/login');
				return;
			} else if (res.status === 404) {
				toast.error('Reset token not found.');
			} else {
				toast.error('Token has expired or already been used.');
			}
		} catch {
			toast.error('Network error. Please try again.');
		} finally {
			loading = false;
		}
	}
</script>

<div class="flex min-h-[calc(100vh-4rem)] items-center justify-center px-4 py-12">
	<Card class="w-full max-w-sm">
		<CardHeader class="text-center">
			<CardTitle class="text-2xl">Reset password</CardTitle>
			<CardDescription>Enter your reset token and new password</CardDescription>
		</CardHeader>
		<CardContent>
			<form onsubmit={handleSubmit} class="flex flex-col gap-4">
				<div class="flex flex-col gap-1.5">
					<Label for="token">Reset Token</Label>
					<Input id="token" placeholder="Paste your token here" bind:value={token} />
					{#if fieldErrors.token}<p class="text-sm text-destructive">{fieldErrors.token}</p>{/if}
				</div>

				<div class="flex flex-col gap-1.5">
					<Label for="newPassword">New Password</Label>
					<PasswordInput id="newPassword" bind:value={newPassword} />
					{#if fieldErrors.newPassword}<p class="text-sm text-destructive">{fieldErrors.newPassword}</p>{/if}
				</div>

				<div class="flex flex-col gap-1.5">
					<Label for="confirmPassword">Confirm Password</Label>
					<PasswordInput id="confirmPassword" bind:value={confirmPassword} />
					{#if fieldErrors.confirmPassword}<p class="text-sm text-destructive">{fieldErrors.confirmPassword}</p>{/if}
				</div>

				<Button type="submit" class="w-full" disabled={loading}>
					{loading ? 'Resetting…' : 'Reset password'}
				</Button>
			</form>
		</CardContent>
	</Card>
</div>
