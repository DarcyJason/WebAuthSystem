<script lang="ts">
    import * as Card from "$lib/components/ui/card/index";
    import * as Field from "$lib/components/ui/field/index";
    import { Input } from "$lib/components/ui/input/index";
    import { Button } from "$lib/components/ui/button/index";
    import { superForm } from "sveltekit-superforms";
    import { zod4 } from "sveltekit-superforms/adapters";
    import { resetPasswordSchema } from "$lib/schema/reset-password.js";
    import { toast } from "svelte-sonner";
    import { untrack } from "svelte";

    let { data } = $props();

    let submitting = $state(false);
    let success = $state(false);

    const { form, enhance } = untrack(() =>
        superForm(data.form, {
            validators: zod4(resetPasswordSchema),
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
                    success = true;
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

<div class="grid grid-cols-3 h-fit">
    <a class="flex justify-center items-center" href="/">
        <img alt="logo" height="64px" src="/logo.svg" width="64px" />
        <span class="text-2xl">Homeryland</span>
    </a>
</div>
<div class="flex min-h-svh w-full items-center justify-center p-6 md:p-10">
    <Card.Root class="mx-auto w-full max-w-sm">
        {#if !data.hasToken}
            <Card.Header>
                <Card.Title class="text-2xl">Invalid Reset Link</Card.Title>
                <Card.Description>
                    This password reset link is invalid or missing. Please request a new one.
                </Card.Description>
            </Card.Header>
            <Card.Content>
                <a href="/forgot-password">
                    <Button class="w-full">Request New Reset Link</Button>
                </a>
            </Card.Content>
        {:else if success}
            <Card.Header>
                <Card.Title class="text-2xl">Password Reset!</Card.Title>
                <Card.Description>
                    Your password has been updated successfully. You can now sign in.
                </Card.Description>
            </Card.Header>
            <Card.Content>
                <a href="/login">
                    <Button class="w-full">Go to Login</Button>
                </a>
            </Card.Content>
        {:else}
            <Card.Header>
                <Card.Title class="text-2xl">Reset Password</Card.Title>
                <Card.Description>
                    Enter your new password below.
                </Card.Description>
            </Card.Header>
            <Card.Content>
                <form method="POST" use:enhance>
                    <input type="hidden" name="token" value={$form.token} />
                    <Field.Group>
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
                            {submitting ? "Resetting..." : "Reset Password"}
                        </Button>
                        <Field.Description class="text-center">
                            Remember your password? <a href="/login">Sign in</a>
                        </Field.Description>
                    </Field.Group>
                </form>
            </Card.Content>
        {/if}
    </Card.Root>
</div>
