<script lang="ts">
    import * as Card from "$lib/components/ui/card/index";
    import * as Field from "$lib/components/ui/field/index";
    import { Input } from "$lib/components/ui/input/index";
    import { Button } from "$lib/components/ui/button/index";
    import { superForm } from "sveltekit-superforms";
    import { zod4 } from "sveltekit-superforms/adapters";
    import { forgotPasswordSchema } from "$lib/schema/forgot-password.js";
    import { toast } from "svelte-sonner";
    import { untrack } from "svelte";

    let { data } = $props();

    let submitting = $state(false);
    let sent = $state(false);

    const { form, enhance } = untrack(() =>
        superForm(data.form, {
            validators: zod4(forgotPasswordSchema),
            clearOnSubmit: "errors-and-message",
            dataType: "json",
            onSubmit() {
                submitting = true;
            },
            onUpdate({ result }) {
                const actionResult = result.data;
                if (!actionResult?.form?.valid) {
                    toast.error("Please enter a valid email address");
                    submitting = false;
                    return;
                }
                if (actionResult.result?.status !== 200) {
                    toast.error(actionResult.result?.message ?? "An error occurred");
                } else {
                    sent = true;
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
        <Card.Header>
            <Card.Title class="text-2xl">Forgot Password</Card.Title>
            <Card.Description>
                {#if sent}
                    Check your inbox for the password reset link.
                {:else}
                    Enter your email and we'll send you a reset link.
                {/if}
            </Card.Description>
        </Card.Header>
        <Card.Content>
            {#if sent}
                <div class="space-y-4">
                    <p class="text-sm text-muted-foreground text-center">
                        Didn't receive the email? Check your spam folder or try again.
                    </p>
                    <Button
                        class="w-full"
                        variant="outline"
                        onclick={() => {
                            sent = false;
                        }}
                    >
                        Try again
                    </Button>
                    <p class="text-sm text-center">
                        <a href="/login" class="underline">Back to Login</a>
                    </p>
                </div>
            {:else}
                <form method="POST" use:enhance>
                    <Field.Group>
                        <Field.Field>
                            <Field.Label for="email">Email</Field.Label>
                            <Input
                                bind:value={$form.email}
                                id="email"
                                name="email"
                                placeholder="m@example.com"
                                required
                                type="email"
                                disabled={submitting}
                            />
                        </Field.Field>
                        <Button class="w-full" type="submit" disabled={submitting}>
                            {submitting ? "Sending..." : "Send Reset Link"}
                        </Button>
                        <Field.Description class="text-center">
                            Remember your password? <a href="/login">Sign in</a>
                        </Field.Description>
                    </Field.Group>
                </form>
            {/if}
        </Card.Content>
    </Card.Root>
</div>
