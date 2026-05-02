<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Card, CardContent, CardHeader, CardTitle } from '$lib/components/ui/card';
	import { Separator } from '$lib/components/ui/separator';

	const features = [
		{
			title: 'Secure Authentication',
			description: 'JWT access tokens stored in memory, refresh tokens in HttpOnly cookies — no XSS risk.'
		},
		{
			title: 'Email Verification',
			description: 'New accounts require email verification before access is granted.'
		},
		{
			title: 'Password Reset',
			description: 'Forgot your password? Reset it securely via a time-limited email link.'
		},
		{
			title: 'Token Rotation',
			description: 'Refresh tokens are rotated on every use, preventing replay attacks.'
		},
		{
			title: 'Change Password',
			description: 'Authenticated users can update their password at any time.'
		},
		{
			title: 'User Profile',
			description: 'Fetch your own profile info with a single authenticated request.'
		}
	];
</script>

<!-- Hero -->
<section class="flex flex-col items-center gap-6 px-4 py-12 text-center">
	<h1 class="text-4xl font-bold tracking-tight sm:text-5xl">WebAuthSystem</h1>
	<p class="max-w-xl text-lg text-muted-foreground">
		A production-ready authentication system built with Rust on the backend and SvelteKit on the
		frontend. Secure, fast, and easy to extend.
	</p>
	<div class="flex flex-wrap justify-center gap-3">
		<Button size="lg" href="/register">Get Started</Button>
		<Button size="lg" variant="outline" href="/login">Sign In</Button>
	</div>
</section>

<Separator />

<!-- Features -->
<section class="mx-auto max-w-6xl px-4 py-20">
	<h2 class="mb-12 text-center text-3xl font-semibold tracking-tight">Features</h2>
	<div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
		{#each features as feature}
			<Card>
				<CardHeader>
					<CardTitle class="text-lg">{feature.title}</CardTitle>
				</CardHeader>
				<CardContent>
					<p class="text-sm text-muted-foreground">{feature.description}</p>
				</CardContent>
			</Card>
		{/each}
	</div>
</section>
