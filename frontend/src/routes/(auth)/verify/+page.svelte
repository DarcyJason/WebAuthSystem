<script lang="ts">
    import * as Card from "$lib/components/ui/card/index";
    import { Input } from "$lib/components/ui/input/index";
    import { Button } from "$lib/components/ui/button/index";
    import * as Field from "$lib/components/ui/field/index";
    import { enhance } from "$app/forms";
    import { toast } from "svelte-sonner";

    let { data, form } = $props();

    let verifying = $state(false);
    let resending = $state(false);

    $effect(() => {
        if (form?.verifyError) {
            toast.error(form.verifyError);
        } else if (form?.success) {
            toast.success(form.message ?? "Verification email sent!");
        } else if (form?.error) {
            toast.error(form.error);
        }
    });
</script>

<div class="grid grid-cols-3 h-fit">
    <a class="flex justify-center items-center" href="/">
        <img alt="logo" height="64px" src="/logo.svg" width="64px" />
        <span class="text-2xl">Homeryland</span>
    </a>
</div>
<div class="flex min-h-svh w-full items-center justify-center px-4">
    <Card.Root class="mx-auto w-full max-w-sm">
        <Card.Header>
            <Card.Title class="text-2xl">Verify Your Email</Card.Title>
            <Card.Description>
                Paste the verification token from your email to activate your account.
            </Card.Description>
            {#if data.email}
                <p class="text-sm text-muted-foreground">
                    Email: <span class="font-medium">{data.email}</span>
                </p>
            {/if}
        </Card.Header>
        <Card.Content class="space-y-5">
            <form
                method="POST"
                action="?/verify"
                use:enhance={() => {
                    verifying = true;
                    return async ({ update }) => {
                        verifying = false;
                        await update();
                    };
                }}
            >
                <Field.Group>
                    <Field.Field>
                        <Field.Label for="token">Verification Token</Field.Label>
                        <Input
                            id="token"
                            name="token"
                            placeholder="e.g. 2565b98c-a542-49f2-b5f0-a3bde220a03b"
                            required
                            type="text"
                            disabled={verifying}
                        />
                    </Field.Field>
                    <Button class="w-full" type="submit" disabled={verifying}>
                        {verifying ? "Verifying..." : "Verify Email"}
                    </Button>
                </Field.Group>
            </form>

            <div class="border-t pt-4">
                <p class="mb-3 text-sm text-muted-foreground">Didn't receive the email? Resend it.</p>
                <form
                    method="POST"
                    action="?/resend"
                    use:enhance={() => {
                        resending = true;
                        return async ({ update }) => {
                            resending = false;
                            await update();
                        };
                    }}
                >
                    <Field.Group>
                        <Field.Field>
                            <Field.Label for="email">Email</Field.Label>
                            <Input
                                id="email"
                                name="email"
                                placeholder="m@example.com"
                                required
                                type="email"
                                value={data.email}
                                disabled={resending}
                            />
                        </Field.Field>
                        <Button class="w-full" type="submit" disabled={resending}>
                            {resending ? "Sending..." : "Resend Verification Email"}
                        </Button>
                        <Field.Description class="text-center">
                            Already verified? <a href="/login">Sign in</a>
                        </Field.Description>
                    </Field.Group>
                </form>
            </div>
        </Card.Content>
    </Card.Root>
</div>
