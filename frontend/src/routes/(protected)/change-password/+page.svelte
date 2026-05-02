<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { accessToken } from '$lib/stores/auth';
	import { get } from 'svelte/store';
	import { Button } from '$lib/components/ui/button';
	import PasswordInput from '$lib/components/PasswordInput.svelte';
	import { Label } from '$lib/components/ui/label';
	import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '$lib/components/ui/card';
	import { changePasswordSchema } from '$lib/schemas';
	import { toast } from 'svelte-sonner';

	let currentPassword = $state('');
	let newPassword = $state('');
	let confirmPassword = $state('');
	let fieldErrors = $state<Record<string, string>>({});
	let loading = $state(false);

	onMount(() => {
		if (!get(accessToken)) goto('/login');
	});

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		fieldErrors = {};

		const result = changePasswordSchema.safeParse({ currentPassword, newPassword, confirmPassword });
		if (!result.success) {
			fieldErrors = Object.fromEntries(result.error.issues.map((i) => [i.path[0], i.message]));
			return;
		}

		loading = true;
		try {
			const res = await fetch('/api/v1/protected/change-password', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					Authorization: `Bearer ${get(accessToken)}`
				},
				body: JSON.stringify(result.data)
			});

			if (res.ok) {
				await fetch('/api/v1/auth/logout', {
					method: 'POST',
					headers: {
						Authorization: `Bearer ${get(accessToken)}`
					}
				});
				toast.success('Password changed. Please sign in again.');
				accessToken.set(null);
				goto('/login');
			} else if (res.status === 401) {
				accessToken.set(null);
				goto('/login');
			} else {
				const data = await res.json().catch(() => ({}));
				toast.error(data?.message ?? 'Invalid current password or passwords do not match.');
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
			<CardTitle class="text-2xl">Change password</CardTitle>
			<CardDescription>Update your account password</CardDescription>
		</CardHeader>
		<CardContent>
			<form onsubmit={handleSubmit} class="flex flex-col gap-4">
				<div class="flex flex-col gap-1.5">
					<Label for="currentPassword">Current Password</Label>
					<PasswordInput id="currentPassword" bind:value={currentPassword} />
					{#if fieldErrors.currentPassword}<p class="text-sm text-destructive">{fieldErrors.currentPassword}</p>{/if}
				</div>

				<div class="flex flex-col gap-1.5">
					<Label for="newPassword">New Password</Label>
					<PasswordInput id="newPassword" bind:value={newPassword} />
					{#if fieldErrors.newPassword}<p class="text-sm text-destructive">{fieldErrors.newPassword}</p>{/if}
				</div>

				<div class="flex flex-col gap-1.5">
					<Label for="confirmPassword">Confirm New Password</Label>
					<PasswordInput id="confirmPassword" bind:value={confirmPassword} />
					{#if fieldErrors.confirmPassword}<p class="text-sm text-destructive">{fieldErrors.confirmPassword}</p>{/if}
				</div>

				<Button type="submit" class="w-full" disabled={loading}>
					{loading ? 'Updating…' : 'Change password'}
				</Button>

				<p class="text-center text-sm text-muted-foreground">
					<a href="/dashboard" class="underline underline-offset-4">Back to dashboard</a>
				</p>
			</form>
		</CardContent>
	</Card>
</div>
