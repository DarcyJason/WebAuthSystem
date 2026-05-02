<script lang="ts">
    import { onMount } from "svelte";
    import { goto } from "$app/navigation";
    import { authenticatedFetch } from "$lib/stores/auth";
    import { PUBLIC_API_BASE_URL } from "$env/static/public";

    const API_BASE_URL = PUBLIC_API_BASE_URL.replace(/\/$/, "");
    let user = $state<{ name?: string; email?: string; status?: string } | null>(null);

    onMount(async () => {
        try {
            const res = await authenticatedFetch(`${API_BASE_URL}/api/v1/protected/me`);
            if (res.status === 401 || !res.ok) {
                await goto("/login");
                return;
            }
            const payload = await res.json().catch(() => null);
            user = payload?.data ?? null;
        } catch {
            await goto("/login");
        }
    });
</script>

<div class="flex h-screen bg-background">
    <aside class="w-64 border-r hidden md:flex flex-col p-4">
        <h2 class="text-xl font-bold mb-6">Dashboard</h2>
        <nav class="space-y-1 flex-1">
            <a class="block px-3 py-2 rounded-md bg-muted font-medium text-sm" href="/dashboard">
                Overview
            </a>
            <a class="block px-3 py-2 rounded-md hover:bg-muted text-sm transition-colors" href="/change-password">
                Change Password
            </a>
        </nav>
        <div class="border-t pt-4">
            <div class="px-3 py-2 mb-2">
                <p class="text-sm font-medium truncate">{user?.name ?? "User"}</p>
                <p class="text-xs text-muted-foreground truncate">{user?.email ?? ""}</p>
            </div>
            <a class="block px-3 py-2 rounded-md hover:bg-destructive/10 text-destructive text-sm transition-colors" href="/logout">
                Logout
            </a>
        </div>
    </aside>

    <main class="flex-1 p-8 overflow-auto">
        <header class="flex justify-between items-center mb-8">
            <div>
                <h1 class="text-3xl font-bold">Welcome back, {user?.name ?? ""}!</h1>
                <p class="text-muted-foreground mt-1">Here's an overview of your account.</p>
            </div>
            <a class="text-sm font-medium text-destructive hover:underline md:hidden" href="/logout">
                Logout
            </a>
        </header>

        <div class="grid gap-4 md:grid-cols-2">
            <div class="p-6 border rounded-lg">
                <h3 class="font-semibold text-lg mb-4">Account Info</h3>
                <dl class="space-y-3 text-sm">
                    <div class="flex gap-3">
                        <dt class="text-muted-foreground w-16 shrink-0">Name</dt>
                        <dd class="font-medium">{user?.name ?? "—"}</dd>
                    </div>
                    <div class="flex gap-3">
                        <dt class="text-muted-foreground w-16 shrink-0">Email</dt>
                        <dd class="font-medium">{user?.email ?? "—"}</dd>
                    </div>
                    <div class="flex gap-3">
                        <dt class="text-muted-foreground w-16 shrink-0">Status</dt>
                        <dd>
                            <span
                                class="inline-flex items-center rounded-full px-2 py-0.5 text-xs font-medium
                                {user?.status === 'active'
                                    ? 'bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400'
                                    : 'bg-yellow-100 text-yellow-700 dark:bg-yellow-900/30 dark:text-yellow-400'}"
                            >
                                {user?.status ?? "—"}
                            </span>
                        </dd>
                    </div>
                </dl>
            </div>

            <div class="p-6 border rounded-lg">
                <h3 class="font-semibold text-lg mb-4">Quick Actions</h3>
                <div class="space-y-2">
                    <a
                        href="/change-password"
                        class="flex items-center gap-2 text-sm px-3 py-2 rounded-md border hover:bg-muted transition-colors"
                    >
                        Change Password
                    </a>
                    <a
                        href="/logout"
                        class="flex items-center gap-2 text-sm px-3 py-2 rounded-md border border-destructive/30 text-destructive hover:bg-destructive/10 transition-colors"
                    >
                        Logout
                    </a>
                </div>
            </div>
        </div>
    </main>
</div>
