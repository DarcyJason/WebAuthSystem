<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import PasswordInput from '$lib/components/PasswordInput.svelte';
	import { Label } from '$lib/components/ui/label';
	import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '$lib/components/ui/card';
	import { accessToken } from '$lib/stores/auth';
	import { goto } from '$app/navigation';
	import { loginSchema } from '$lib/schemas';
	import { toast } from 'svelte-sonner';

	let nameOrEmail = $state('');
	let password = $state('');
	let fieldErrors = $state<Record<string, string>>({});
	let loading = $state(false);

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		fieldErrors = {};

		const result = loginSchema.safeParse({ nameOrEmail, password });
		if (!result.success) {
			fieldErrors = Object.fromEntries(
				result.error.issues.map((i) => [i.path[0], i.message])
			);
			return;
		}

		loading = true;
		try {
			const res = await fetch('/api/v1/auth/login', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				credentials: 'include',
				body: JSON.stringify(result.data)
			});

			if (res.ok) {
				const token = res.headers.get('Authorization')?.replace('Bearer ', '') ?? null;
				accessToken.set(token);
				goto('/dashboard');
			} else if (res.status === 404) {
				toast.error('User not found.');
			} else {
				const data = await res.json().catch(() => ({}));
				toast.error(data?.message ?? 'Invalid credentials or email not verified.');
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
			<CardTitle class="text-2xl">Sign in</CardTitle>
			<CardDescription>Enter your name or email and password</CardDescription>
		</CardHeader>
		<CardContent>
			<form onsubmit={handleSubmit} class="flex flex-col gap-4">
				<div class="flex flex-col gap-1.5">
					<Label for="nameOrEmail">Name or Email</Label>
					<Input id="nameOrEmail" type="text" placeholder="John Doe or you@example.com" bind:value={nameOrEmail} />
					{#if fieldErrors.nameOrEmail}<p class="text-sm text-destructive">{fieldErrors.nameOrEmail}</p>{/if}
				</div>

				<div class="flex flex-col gap-1.5">
					<div class="flex items-center justify-between">
						<Label for="password">Password</Label>
						<a href="/forgot-password" class="text-sm underline underline-offset-4">Forgot password?</a>
					</div>
					<PasswordInput id="password" bind:value={password} />
					{#if fieldErrors.password}<p class="text-sm text-destructive">{fieldErrors.password}</p>{/if}
				</div>

				<Button type="submit" class="w-full" disabled={loading}>
					{loading ? 'Signing in…' : 'Sign in'}
				</Button>

				<p class="text-center text-sm text-muted-foreground">
					Don't have an account? <a href="/register" class="underline underline-offset-4">Register</a>
				</p>
			</form>
		</CardContent>
	</Card>
</div>
