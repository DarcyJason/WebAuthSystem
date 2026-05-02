<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '$lib/components/ui/card';
	import { resendVerificationSchema } from '$lib/schemas';
	import { page } from '$app/state';
	import { toast } from 'svelte-sonner';

	const prefillEmail = page.url.searchParams.get('email') ?? '';

	let email = $state(prefillEmail);
	let fieldErrors = $state<Record<string, string>>({});
	let loading = $state(false);

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		fieldErrors = {};

		const result = resendVerificationSchema.safeParse({ email });
		if (!result.success) {
			fieldErrors = Object.fromEntries(result.error.issues.map((i) => [i.path[0], i.message]));
			return;
		}

		loading = true;
		try {
			const res = await fetch('/api/v1/auth/resend-verification', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify(result.data)
			});

			if (res.ok) {
				toast.success('If your account exists and is unverified, a new verification email has been sent.');
			} else {
				const data = await res.json().catch(() => ({}));
				toast.error(data?.message ?? 'Validation error. Please check your input.');
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
			<CardTitle class="text-2xl">Resend verification</CardTitle>
			<CardDescription>Enter your email to receive a new verification link</CardDescription>
		</CardHeader>
		<CardContent>
			<form onsubmit={handleSubmit} class="flex flex-col gap-4">
				<div class="flex flex-col gap-1.5">
					<Label for="email">Email</Label>
					<Input id="email" type="email" placeholder="you@example.com" bind:value={email} />
					{#if fieldErrors.email}<p class="text-sm text-destructive">{fieldErrors.email}</p>{/if}
				</div>

				<Button type="submit" class="w-full" disabled={loading}>
					{loading ? 'Sending…' : 'Resend verification email'}
				</Button>

				<p class="text-center text-sm text-muted-foreground">
					<a href="/login" class="underline underline-offset-4">Back to sign in</a>
				</p>
			</form>
		</CardContent>
	</Card>
</div>
