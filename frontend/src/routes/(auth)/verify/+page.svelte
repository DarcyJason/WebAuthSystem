<script lang="ts">
    import * as Card from "$lib/components/ui/card/index";
    import {Input} from "$lib/components/ui/input/index";
    import {Button} from "$lib/components/ui/button/index";
    import * as Field from "$lib/components/ui/field/index";
    import {z} from "zod";
    import {toast} from "svelte-sonner";

    const API_BASE_URL = import.meta.env.VITE_API_BASE_URL;

    const schema = z.object({
        email: z.string().email("Invalid email address"),
    });

    let email = $state("");
    let isLoading = $state(false);
    let fieldErrors = $state<{ email?: { message?: string }[] }>({});

    async function handleResend(e: SubmitEvent) {
        e.preventDefault();
        fieldErrors = {};

        const parsed = schema.safeParse({ email });
        if (!parsed.success) {
            fieldErrors.email = parsed.error.issues.map(i => ({ message: i.message }));
            return;
        }

        try {
            isLoading = true;
            const res = await fetch(`${API_BASE_URL}/auth/resend-verification`, {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ email: email }),
            });

            const data = await res.json().catch(() => ({}));
            if (res.ok) {
                toast.success("Verification email sent successfully");
            } else {
                toast.error(data.message ?? "Failed to resend verification");
            }
        } catch (err) {
            toast.error("Network error");
        } finally {
            isLoading = false;
        }
    }
</script>

<div class="flex h-screen w-full items-center justify-center px-4">
    <Card.Root class="mx-auto w-full max-w-sm">
        <Card.Header>
            <Card.Title class="text-2xl">Resend Verification</Card.Title>
            <Card.Description>Enter your email to receive a new verification link.</Card.Description>
        </Card.Header>
        <Card.Content>
            <form onsubmit={handleResend}>
                <Field.Group>
                    <Field.Field>
                        <Field.Label for="email">Email</Field.Label>
                        <Input bind:value={email} id="email" placeholder="m@example.com" required type="email" />
                        <Field.Error errors={fieldErrors.email} />
                    </Field.Field>
                    <Button class="w-full" type="submit" disabled={isLoading}>
                        {isLoading ? "Sending..." : "Resend Verification Email"}
                    </Button>
                </Field.Group>
            </form>
        </Card.Content>
    </Card.Root>
</div>
