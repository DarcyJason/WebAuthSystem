<script lang="ts">
    import * as Card from "$lib/components/ui/card/index";
    import * as Field from "$lib/components/ui/field/index";
    import { Input } from "$lib/components/ui/input/index";
    import { Button } from "$lib/components/ui/button/index";
    import { superForm } from "sveltekit-superforms";
    import { zod4 } from "sveltekit-superforms/adapters";
    import { changePasswordSchema } from "$lib/schema/change-password.js";
    import { toast } from "svelte-sonner";
    import { untrack } from "svelte";

    let { data } = $props();

    let submitting = $state(false);
    let changed = $state(false);

    const { form, enhance } = untrack(() =>
        superForm(data.form, {
            validators: zod4(changePasswordSchema),
            clearOnSubmit: "errors-and-message",
            dataType: "json",
            onSubmit() {
                submitting = true;
            },
            onUpdate({ result }) {
                const actionResult = result.data;
                if (!actionResult?.form?.valid) {
                    toast.error("Please check the form for errors");
                    submitting = false;
                    return;
                }
                if (actionResult.result?.status !== 200) {
                    toast.error(actionResult.result?.message ?? "An error occurred");
                } else {
                    changed = true;
                    toast.success(actionResult.result.message);
                }
                submitting = false;
            },
            onError() {
                submitting = false;
                toast.error("An unexpected error occurred.");
            },
        }),
    );
</script>

<div class="flex h-screen bg-background">
    <aside class="w-64 border-r hidden md:flex flex-col p-4">
        <h2 class="text-xl font-bold mb-6">Dashboard</h2>
        <nav class="space-y-1 flex-1">
            <a class="block px-3 py-2 rounded-md hover:bg-muted text-sm transition-colors" href="/dashboard">
                Overview
            </a>
            <a class="block px-3 py-2 rounded-md bg-muted font-medium text-sm" href="/change-password">
                Change Password
            </a>
        </nav>
        <div class="border-t pt-4">
            <a class="block px-3 py-2 rounded-md hover:bg-destructive/10 text-destructive text-sm transition-colors" href="/logout">
                Logout
            </a>
        </div>
    </aside>

    <main class="flex-1 p-8 overflow-auto">
        <header class="flex items-center gap-4 mb-8">
            <a href="/dashboard" class="text-muted-foreground hover:text-foreground text-sm transition-colors">
                ← Back
            </a>
            <h1 class="text-3xl font-bold">Change Password</h1>
        </header>

        <div class="max-w-md">
            {#if changed}
                <Card.Root>
                    <Card.Header>
                        <Card.Title>Password Updated!</Card.Title>
                        <Card.Description>
                            Your password has been changed successfully.
                        </Card.Description>
                    </Card.Header>
                    <Card.Content>
                        <a href="/dashboard">
                            <Button class="w-full">Back to Dashboard</Button>
                        </a>
                    </Card.Content>
                </Card.Root>
            {:else}
                <Card.Root>
                    <Card.Header>
                        <Card.Title>Change Password</Card.Title>
                        <Card.Description>
                            Enter your current password and choose a new one.
                        </Card.Description>
                    </Card.Header>
                    <Card.Content>
                        <form method="POST" use:enhance>
                            <Field.Group>
                                <Field.Field>
                                    <Field.Label for="currentPassword">Current Password</Field.Label>
                                    <Input
                                        bind:value={$form.currentPassword}
                                        id="currentPassword"
                                        name="currentPassword"
                                        required
                                        type="password"
                                        disabled={submitting}
                                    />
                                </Field.Field>
                                <Field.Field>
                                    <Field.Label for="newPassword">New Password</Field.Label>
                                    <Input
                                        bind:value={$form.newPassword}
                                        id="newPassword"
                                        name="newPassword"
                                        required
                                        type="password"
                                        disabled={submitting}
                                    />
                                    <Field.Description>Must be 8 - 128 characters long.</Field.Description>
                                </Field.Field>
                                <Field.Field>
                                    <Field.Label for="confirmPassword">Confirm New Password</Field.Label>
                                    <Input
                                        bind:value={$form.confirmPassword}
                                        id="confirmPassword"
                                        name="confirmPassword"
                                        required
                                        type="password"
                                        disabled={submitting}
                                    />
                                </Field.Field>
                                <Button class="w-full" type="submit" disabled={submitting}>
                                    {submitting ? "Updating..." : "Update Password"}
                                </Button>
                            </Field.Group>
                        </form>
                    </Card.Content>
                </Card.Root>
            {/if}
        </div>
    </main>
</div>
