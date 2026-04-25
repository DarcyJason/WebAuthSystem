<script lang="ts">
</script>

<div class="flex h-screen bg-background">
    <!-- Sidebar -->
    <aside class="w-64 border-r hidden md:block p-4">
        <h2 class="text-xl font-bold mb-6">Dashboard</h2>
        <nav class="space-y-2">
            <a class="block p-2 hover:bg-muted rounded" href="/dashboard">Overview</a>
            <a class="block p-2 hover:bg-muted rounded" href="/profile">Profile</a>
            <a class="block p-2 text-destructive hover:bg-destructive/10 rounded" href="/logout">Logout</a>
        </nav>
    </aside>

    <!-- Main Content -->
    <main class="flex-1 p-8">
        <header class="flex justify-between items-center mb-8">
            <h1 class="text-3xl font-bold">Welcome back!</h1>
            <a class="text-sm font-medium hover:underline" href="/logout">Logout</a>
        </header>

        <div class="grid gap-4">
            <div class="p-6 border rounded-lg">
                <h3 class="font-semibold text-lg">System Status</h3>
                <p class="text-muted-foreground">Everything is running smoothly.</p>
            </div>
        </div>
    </main>
</div>
