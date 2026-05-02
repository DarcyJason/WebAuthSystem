<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import PasswordInput from '$lib/components/PasswordInput.svelte';
	import { Label } from '$lib/components/ui/label';
	import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '$lib/components/ui/card';
	import { goto } from '$app/navigation';
	import { registerSchema } from '$lib/schemas';
	import { toast } from 'svelte-sonner';

	let name = $state('');
	let email = $state('');
	let password = $state('');
	let confirmPassword = $state('');
	let fieldErrors = $state<Record<string, string>>({});
	let loading = $state(false);

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		fieldErrors = {};

		const result = registerSchema.safeParse({ name, email, password, confirmPassword });
		if (!result.success) {
			fieldErrors = Object.fromEntries(
				result.error.issues.map((i) => [i.path[0], i.message])
			);
			return;
		}

		loading = true;
		try {
			const res = await fetch('/api/v1/auth/register', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify(result.data)
			});

			if (res.ok) {
				toast.success('Registration successful. Please verify your email.');
				goto('/verify');
			} else if (res.status === 409) {
				toast.error('An account with this email already exists.');
			} else {
				const data = await res.json().catch(() => ({}));
				toast.error(data?.message ?? 'Validation error. Please check your inputs.');
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
			<CardTitle class="text-2xl">Create an account</CardTitle>
			<CardDescription>Enter your details to get started</CardDescription>
		</CardHeader>
		<CardContent>
			<form onsubmit={handleSubmit} class="flex flex-col gap-4">
				<div class="flex flex-col gap-1.5">
					<Label for="name">Name</Label>
					<Input id="name" type="text" placeholder="John Doe" bind:value={name} />
					{#if fieldErrors.name}<p class="text-sm text-destructive">{fieldErrors.name}</p>{/if}
				</div>

				<div class="flex flex-col gap-1.5">
					<Label for="email">Email</Label>
					<Input id="email" type="email" placeholder="you@example.com" bind:value={email} />
					{#if fieldErrors.email}<p class="text-sm text-destructive">{fieldErrors.email}</p>{/if}
				</div>

				<div class="flex flex-col gap-1.5">
					<Label for="password">Password</Label>
					<PasswordInput id="password" bind:value={password} />
					{#if fieldErrors.password}<p class="text-sm text-destructive">{fieldErrors.password}</p>{/if}
				</div>

				<div class="flex flex-col gap-1.5">
					<Label for="confirmPassword">Confirm Password</Label>
					<PasswordInput id="confirmPassword" bind:value={confirmPassword} />
					{#if fieldErrors.confirmPassword}<p class="text-sm text-destructive">{fieldErrors.confirmPassword}</p>{/if}
				</div>

				<Button type="submit" class="w-full" disabled={loading}>
					{loading ? 'Registering…' : 'Register'}
				</Button>

				<p class="text-center text-sm text-muted-foreground">
					Already have an account? <a href="/login" class="underline underline-offset-4">Sign in</a>
				</p>
			</form>
		</CardContent>
	</Card>
</div>
