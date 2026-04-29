<script lang="ts">
    import * as Card from "$lib/components/ui/card/index";
    import { Input } from "$lib/components/ui/input/index";
    import { Button } from "$lib/components/ui/button/index";
    import * as Field from "$lib/components/ui/field/index";
    import { superForm } from "sveltekit-superforms";
    import { zod4 } from "sveltekit-superforms/adapters";
    import { loginSchema } from "$lib/schema/login.js";
    import { untrack } from "svelte";
    import { toast } from "svelte-sonner";
    import { accessToken, logout } from "$lib/authStore";
    import { goto } from "$app/navigation";
    import { onMount } from "svelte";

    let { data } = $props();

    onMount(() => {
        logout();
    });

    let submitting = $state(false);

    const { form, enhance } = untrack(() =>
        superForm(data.form, {
            validators: zod4(loginSchema),
            clearOnSubmit: "errors-and-message",
            dataType: "json",
            onSubmit() {
                submitting = true;
            },
            onUpdate({ result }) {
                const actionResult = result.data;
                if (!actionResult.form.valid) {
                    toast.error("Please check the form for errors");
                    submitting = false;
                    return;
                }
                if (actionResult.result.status != 200) {
                    toast.error(actionResult.result.message);
                } else {
                    toast.success(actionResult.result.message);
                    if (actionResult.result.accessToken) {
                        accessToken.set(actionResult.result.accessToken);
                    }
                    goto("/dashboard");
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
<div class="flex h-screen w-full items-center justify-center px-4">
    <Card.Root class="mx-auto w-full max-w-sm">
        <Card.Header>
            <Card.Title class="text-2xl">Login</Card.Title>
            <Card.Description>
                Enter your email below to login to your account
            </Card.Description>
        </Card.Header>
        <Card.Content>
            <form method="POST" use:enhance>
                <Field.Group>
                    <Field.Field>
                        <Field.Label for="nameOrEmail"
                            >Name or Email</Field.Label
                        >
                        <Input
                            bind:value={$form.nameOrEmail}
                            id="nameOrEmail"
                            name="nameOrEmail"
                            placeholder="Enter your name or email"
                            required
                            type="text"
                            disabled={submitting}
                        />
                    </Field.Field>
                    <Field.Field>
                        <div class="flex items-center">
                            <Field.Label for="password">Password</Field.Label>
                            <a
                                class="ms-auto inline-block text-sm underline"
                                href="/forgot-password"
                            >
                                Forgot your password?
                            </a>
                        </div>
                        <Input
                            bind:value={$form.password}
                            id="password"
                            name="password"
                            required
                            type="password"
                            disabled={submitting}
                        />
                    </Field.Field>
                    <Field.Field>
                        <Button class="w-full" type="submit">
                            {submitting ? "Login in..." : "Login"}
                        </Button>
                        <Field.Description class="text-center">
                            Don't have an account? <a href="/register"
                                >Sign up</a
                            >
                        </Field.Description>
                    </Field.Field>
                </Field.Group>
            </form>
        </Card.Content>
    </Card.Root>
</div>
