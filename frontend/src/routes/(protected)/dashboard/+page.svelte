<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { accessToken } from '$lib/stores/auth';
	import { get } from 'svelte/store';
	import { Button } from '$lib/components/ui/button';
	import { Card, CardContent, CardHeader, CardTitle } from '$lib/components/ui/card';
	import { Separator } from '$lib/components/ui/separator';
	import { toast } from 'svelte-sonner';

	type UserInfo = { userId: string; name: string; email: string; status: string };

	let user = $state<UserInfo | null>(null);

	onMount(async () => {
		const token = get(accessToken);
		if (!token) { goto('/login'); return; }

		try {
			const res = await fetch('/api/v1/protected/me', {
				headers: { Authorization: `Bearer ${token}` }
			});

			if (res.ok) {
				const data = await res.json();
				user = data.data ?? data;
			} else if (res.status === 401) {
				accessToken.set(null);
				goto('/login');
			} else {
				toast.error('Failed to load user info.');
			}
		} catch {
			toast.error('Network error. Please try again.');
		}
	});

	async function logout() {
		const token = get(accessToken);
		await fetch('/api/v1/auth/logout', {
			method: 'POST',
			headers: { Authorization: `Bearer ${token}` },
			credentials: 'include'
		}).catch(() => {});
		accessToken.set(null);
		goto('/login');
	}
</script>

<div class="mx-auto max-w-lg px-4 py-16">
	<h1 class="mb-8 text-3xl font-bold">Dashboard</h1>

	{#if !user}
		<p class="text-muted-foreground">Loading…</p>
	{:else}
		<Card>
			<CardHeader>
				<CardTitle>Profile</CardTitle>
			</CardHeader>
			<CardContent class="flex flex-col gap-3 text-sm">
				<div class="flex justify-between">
					<span class="text-muted-foreground">User ID</span>
					<span class="font-mono">{user.userId}</span>
				</div>
				<Separator />
				<div class="flex justify-between">
					<span class="text-muted-foreground">Name</span>
					<span>{user.name}</span>
				</div>
				<Separator />
				<div class="flex justify-between">
					<span class="text-muted-foreground">Email</span>
					<span>{user.email}</span>
				</div>
				<Separator />
				<div class="flex justify-between">
					<span class="text-muted-foreground">Status</span>
					<span class="capitalize">{user.status}</span>
				</div>
			</CardContent>
		</Card>

		<div class="mt-6 flex flex-col gap-3">
			<Button variant="outline" onclick={() => goto('/change-password')}>Change password</Button>
			<Button variant="destructive" onclick={logout}>Logout</Button>
		</div>
	{/if}
</div>
