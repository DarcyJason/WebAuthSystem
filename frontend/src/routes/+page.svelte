<script lang="ts">
	import { Button } from '$lib/components/ui/button';
</script>

<header class="absolute top-0 left-0 p-4 md:p-6">
	<a href="/" aria-label="Back to homepage">
		<h1 class="text-2xl font-bold text-slate-800">Homeryland</h1>
	</a>
</header>

<div
	class="flex h-screen w-full flex-col bg-cover bg-center"
	style="background-image: url('index.jpg');"
>
	<div class="flex justify-end space-x-2 p-4 md:p-6">
		<a href="/register">
			<Button variant="ghost" class="w-24">Sign Up</Button>
		</a> <a href="/login"> <Button class="w-24">Login</Button> </a>
	</div>
	<main class="flex flex-1 flex-col items-center justify-center text-center">
		<div class="space-y-4">
			<h2 class="text-4xl font-bold tracking-tighter text-slate-800 sm:text-5xl md:text-6xl">
				Secure & Fast Authentication System
			</h2>
			<p class="text-slate-600 md:text-xl">
				Built with <a href="https://svelte.dev" target="_blank" class="underline">Svelte</a>
				and
				<a href="https://github.com/tokio-rs/axum" target="_blank" class="underline">Axum</a> for a seamless
				and performant user experience.
			</p>
		</div>
	</main>
</div>
