<script lang="ts">
    import * as Card from "$lib/components/ui/card/index";
    import * as Field from "$lib/components/ui/field/index";
    import { Input } from "$lib/components/ui/input/index";
    import { Button } from "$lib/components/ui/button/index";
    import { superForm } from "sveltekit-superforms";
    import { registerSchema } from "$lib/schema/register.js";
    import { zod4 } from "sveltekit-superforms/adapters";
    import { toast } from "svelte-sonner";
    import { untrack } from "svelte";
    import { goto } from "$app/navigation";

    let { data } = $props();

    let submitting = $state(false);
    let registered = $state(false);
    let submittedEmail = $state("");

    const { form, enhance } = untrack(() =>
        superForm(data.form, {
            validators: zod4(registerSchema),
            clearOnSubmit: "errors-and-message",
            dataType: "json",
            onSubmit() {
                submitting = true;
                submittedEmail = $form.email ?? "";
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
                    registered = true;
                    toast.success(actionResult.result.message);
                    goto(`/verify?email=${encodeURIComponent(submittedEmail)}`);
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
    <div class="w-full max-w-sm">
        {#if registered}
            <Card.Root>
                <Card.Header>
                    <Card.Title>Check Your Email</Card.Title>
                    <Card.Description>
                        We've sent a verification link to <strong>{submittedEmail}</strong>.
                        Please check your inbox and click the link to activate your account.
                    </Card.Description>
                </Card.Header>
                <Card.Content class="space-y-3">
                    <p class="text-sm text-muted-foreground">
                        Didn't receive the email? Check your spam folder or request a new link.
                    </p>
                    <a href="/verify">
                        <Button variant="outline" class="w-full">Resend Verification Email</Button>
                    </a>
                    <a href="/login">
                        <Button class="w-full">Go to Login</Button>
                    </a>
                </Card.Content>
            </Card.Root>
        {:else}
            <Card.Root>
                <Card.Header>
                    <Card.Title>Create an account</Card.Title>
                    <Card.Description>
                        Enter your information below to create your account
                    </Card.Description>
                </Card.Header>
                <Card.Content>
                    <form method="POST" use:enhance>
                        <Field.Group>
                            <Field.Field>
                                <Field.Label for="name">Full Name</Field.Label>
                                <Input
                                    bind:value={$form.name}
                                    id="name"
                                    name="name"
                                    placeholder="Darcy Jason"
                                    required
                                    type="text"
                                    disabled={submitting}
                                />
                            </Field.Field>
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
                                <Field.Description>
                                    We'll use this to contact you. We will not share
                                    your email with anyone else.
                                </Field.Description>
                            </Field.Field>
                            <Field.Field>
                                <Field.Label for="password">Password</Field.Label>
                                <Input
                                    bind:value={$form.password}
                                    id="password"
                                    name="password"
                                    required
                                    type="password"
                                    disabled={submitting}
                                />
                                <Field.Description>Must be 8 - 128 characters long.</Field.Description>
                            </Field.Field>
                            <Field.Field>
                                <Field.Label for="confirm-password">Confirm Password</Field.Label>
                                <Input
                                    bind:value={$form.confirmPassword}
                                    id="confirm-password"
                                    name="confirmPassword"
                                    required
                                    type="password"
                                    disabled={submitting}
                                />
                                <Field.Description>Please confirm your password.</Field.Description>
                            </Field.Field>
                            <Field.Field>
                                <Button disabled={submitting} type="submit" class="w-full">
                                    {submitting ? "Creating account..." : "Create account"}
                                </Button>
                                <Field.Description class="px-6 text-center">
                                    Already have an account? <a href="/login">Sign in</a>
                                </Field.Description>
                            </Field.Field>
                        </Field.Group>
                    </form>
                </Card.Content>
            </Card.Root>
        {/if}
    </div>
</div>
