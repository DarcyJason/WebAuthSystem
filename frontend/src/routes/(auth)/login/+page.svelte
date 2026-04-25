<script lang="ts">
    import * as Card from "$lib/components/ui/card/index";
    import {Input} from "$lib/components/ui/input/index";
    import {Button} from "$lib/components/ui/button/index";
    import * as Field from "$lib/components/ui/field/index";
    import {z} from "zod";
    import {toast} from "svelte-sonner";

    const API_BASE_URL = import.meta.env.VITE_API_BASE_URL;

    const loginSchema = z
        .object({
            nameOrEmail: z
                .string()
                .min(2, "Name or Email must be at least 2 characters")
                .max(100, "Name or Email must be less than 100 characters"),
            password: z
                .string()
                .min(8, "Password must be 8 - 128 characters long")
                .max(128, "Password is too long")
                .regex(
                    /[A-Z]/,
                    "Password must contain at least one uppercase letter",
                )
                .regex(
                    /[a-z]/,
                    "Password must contain at least one lowercase letter",
                )
                .regex(/[0-9]/, "Password must contain at least one number")
                .regex(
                    /[^A-Za-z0-9]/,
                    "Password must contain at least one special character",
                ),
        });

    let nameOrEmail = $state("");
    let password = $state("");
    let isLoading = $state(false);
    type LoginField = "nameOrEmail" | "password";
    type LoginFormErrors = Partial<Record<LoginField, { message?: string }[]>>;
    let fieldErrors = $state<LoginFormErrors>({});

    async function handleSubmit(e: SubmitEvent) {
        e.preventDefault();

        if (!API_BASE_URL) {
            toast.error("Missing VITE_API_BASE_URL in .env");
            return;
        }

        fieldErrors = {};

        const parsed = loginSchema.safeParse({
            nameOrEmail,
            password,
        });
        if (!parsed.success) {
            const nextErrors: LoginFormErrors = {};
            for (const issue of parsed.error.issues) {
                const key = issue.path[0];
                if (
                    key === "nameOrEmail" ||
                    key === "password"
                ) {
                    nextErrors[key] ??= [];
                    nextErrors[key].push({message: issue.message});
                }
            }
            fieldErrors = nextErrors;
            toast.error("Please fix the form errors");
            return;
        }

        try {
            isLoading = true;
            const payload = parsed.data;
            nameOrEmail = payload.nameOrEmail;
            password = payload.password;

            const res = await fetch(`${API_BASE_URL}/auth/login`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify(payload),
            });

            const data = await res.json().catch(() => {
            });
            const message =
                data?.message ??
                (res.ok ? "Login success" : "Login failed");
            if (!res.ok) {
                toast.error(message);
                return;
            }
            toast.success(message);
        } catch (err) {
            const message =
                err instanceof Error ? err.message : "Network error";
            toast.error(message);
        } finally {
            isLoading = false;
        }
    }
</script>

<div class="grid grid-cols-3 h-fit">
    <a class="flex justify-center items-center" href="/">
        <img alt="logo" height="64px" src="/logo.svg" width="64px"/>
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
            <form onsubmit={handleSubmit}>
                <Field.Group>
                    <Field.Field>
                        <Field.Label for="nameOrEmail">Name or Email</Field.Label>
                        <Input
                                bind:value={nameOrEmail}
                                id="nameOrEmail"
                                name="nameOrEmail"
                                placeholder="Enter your name or email"
                                required
                                type="text"
                        />
                        <Field.Error errors={fieldErrors.nameOrEmail}/>
                    </Field.Field>
                    <Field.Field>
                        <div class="flex items-center">
                            <Field.Label for="password">Password
                            </Field.Label>
                            <a class="ms-auto inline-block text-sm underline" href="/forgot-password">
                                Forgot your password?
                            </a>
                        </div>
                        <Input
                                bind:value={password}
                                id="password"
                                name="password"
                                required
                                type="password"
                        />
                        <Field.Error errors={fieldErrors.password}/>
                    </Field.Field>
                    <Field.Field>
                        <Button class="w-full" type="submit">
                            {isLoading ? "Login in..." : "Login"}
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
