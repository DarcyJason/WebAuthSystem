<script lang="ts">
	import { page } from '$app/state';
	import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';

	const email = page.url.searchParams.get('email') ?? '';
</script>

<div class="flex min-h-[calc(100vh-4rem)] items-center justify-center px-4 py-12">
	<Card class="w-full max-w-sm text-center">
		<CardHeader>
			<CardTitle class="text-2xl">Check your email</CardTitle>
			<CardDescription>
				We sent a verification link to{email ? ` ${email}` : ' your email address'}.
				Please click the link to activate your account.
			</CardDescription>
		</CardHeader>
		<CardContent class="flex flex-col gap-3">
			<p class="text-sm text-muted-foreground">Didn't receive it?</p>
			<Button variant="outline" href="/resend-verification{email ? `?email=${encodeURIComponent(email)}` : ''}">
				Resend verification email
			</Button>
			<a href="/login" class="text-sm underline underline-offset-4">Back to sign in</a>
		</CardContent>
	</Card>
</div>
