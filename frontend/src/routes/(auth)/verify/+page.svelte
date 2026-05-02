<script lang="ts">
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { toast } from 'svelte-sonner';

	const urlToken = page.url.searchParams.get('token') ?? '';

	let token = $state(urlToken);
	let status: 'idle' | 'verifying' = $state('idle');

	async function verify() {
		if (!token.trim()) { toast.error('Please enter the verification token.'); return; }
		status = 'verifying';
		try {
			const res = await fetch('/api/v1/auth/verify', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ token: token.trim() })
			});
			if (res.ok) {
				toast.success('Email verified. Please sign in.');
				goto('/login');
				return;
			} else if (res.status === 404) {
				toast.error('Verification token not found.');
			} else {
				toast.error('Token has expired or already been used.');
			}
		} catch {
			toast.error('Network error. Please try again.');
		} finally {
			status = 'idle';
		}
	}

</script>

<div class="flex min-h-[calc(100vh-4rem)] items-center justify-center px-4 py-12">
	<Card class="w-full max-w-sm">
		<CardHeader class="text-center">
			<CardTitle class="text-2xl">Verify your email</CardTitle>
			<CardDescription>Enter the token from your verification email</CardDescription>
		</CardHeader>
		<CardContent class="flex flex-col gap-4">
			<div class="flex flex-col gap-1.5">
				<Label for="token">Verification Token</Label>
				<Input id="token" placeholder="Paste your token here" bind:value={token} />
			</div>

			<Button class="w-full" disabled={status === 'verifying'} onclick={verify}>
				{status === 'verifying' ? 'Verifying…' : 'Verify'}
			</Button>

			<p class="text-center text-sm text-muted-foreground">
				Didn't receive an email?
				<a href="/resend-verification" class="underline underline-offset-4">Resend</a>
			</p>
		</CardContent>
	</Card>
</div>
