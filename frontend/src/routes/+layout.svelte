<script lang="ts">
	import './layout.css';
	import { accessToken } from '$lib/stores/auth';
	import { Button } from '$lib/components/ui/button';
	import { Toaster } from '$lib/components/ui/sonner';

	let { children } = $props();
</script>

<div class="flex min-h-screen flex-col">
	<nav class="sticky top-0 z-50 border-b border-border bg-background/80 backdrop-blur">
		<div class="mx-auto flex h-16 w-full max-w-6xl items-center justify-between px-4">
			<a href="/" class="flex items-center gap-2">
				<img src="/logo.svg" alt="WebAuthSystem" class="h-8 w-auto" />
				<span class="text-base font-semibold tracking-wide">Homeryland</span>
			</a>
			<div class="flex items-center gap-2">
				{#if $accessToken}
					<Button variant="ghost" href="/dashboard">Dashboard</Button>
					<Button variant="outline" onclick={() => accessToken.set(null)}>Logout</Button>
				{:else}
					<Button variant="ghost" href="/login">Login</Button>
					<Button href="/register">Register</Button>
				{/if}
			</div>
		</div>
	</nav>

	<main class="flex-1">
		{@render children()}
	</main>

	<footer class="border-t border-border">
		<div class="mx-auto flex h-14 w-full max-w-6xl items-center px-4 text-sm text-muted-foreground">
			<span>© {new Date().getFullYear()} Homeryland. All rights reserved.</span>
		</div>
	</footer>
</div>

<Toaster position="bottom-right" duration={3000} richColors />
