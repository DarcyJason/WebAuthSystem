<script lang="ts">
    import * as Card from "$lib/components/ui/card/index";
    import { Input } from "$lib/components/ui/input/index";
    import { Button } from "$lib/components/ui/button/index";
    import * as Field from "$lib/components/ui/field/index";
    import { enhance } from "$app/forms";
    import { toast } from "svelte-sonner";

    let { data, form } = $props();

    let isLoading = $state(false);

    $effect(() => {
        if (form?.success) {
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
    {#if data.verified === true}
        <Card.Root class="mx-auto w-full max-w-sm text-center">
            <Card.Header>
                <Card.Title class="text-2xl">Email Verified!</Card.Title>
                <Card.Description>{data.message}</Card.Description>
            </Card.Header>
            <Card.Content>
                <a href="/login">
                    <Button class="w-full">Go to Login</Button>
                </a>
            </Card.Content>
        </Card.Root>
    {:else if data.verified === false}
        <Card.Root class="mx-auto w-full max-w-sm text-center">
            <Card.Header>
                <Card.Title class="text-2xl text-destructive">Verification Failed</Card.Title>
                <Card.Description>{data.message}</Card.Description>
            </Card.Header>
            <Card.Content class="space-y-3">
                <p class="text-sm text-muted-foreground">
                    The link may be expired or already used. Please request a new one.
                </p>
                <a href="/verify">
                    <Button variant="outline" class="w-full">Request New Verification Link</Button>
                </a>
            </Card.Content>
        </Card.Root>
    {:else}
        <Card.Root class="mx-auto w-full max-w-sm">
            <Card.Header>
                <Card.Title class="text-2xl">Verify Your Email</Card.Title>
                <Card.Description>
                    Enter your email address to receive a new verification link.
                </Card.Description>
            </Card.Header>
            <Card.Content>
                {#if form?.success}
                    <div class="mb-4 rounded-md bg-green-50 dark:bg-green-950 p-3 text-sm text-green-700 dark:text-green-300">
                        {form.message}
                    </div>
                {/if}
                <form
                    method="POST"
                    action="?/resend"
                    use:enhance={() => {
                        isLoading = true;
                        return async ({ update }) => {
                            isLoading = false;
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
                                disabled={isLoading}
                            />
                        </Field.Field>
                        <Button class="w-full" type="submit" disabled={isLoading}>
                            {isLoading ? "Sending..." : "Resend Verification Email"}
                        </Button>
                        <Field.Description class="text-center">
                            Already verified? <a href="/login">Sign in</a>
                        </Field.Description>
                    </Field.Group>
                </form>
            </Card.Content>
        </Card.Root>
    {/if}
</div>
