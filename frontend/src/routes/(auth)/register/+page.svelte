<script lang="ts">
    import * as Card from "$lib/components/ui/card/index";
    import * as Field from "$lib/components/ui/field/index";
    import {Input} from "$lib/components/ui/input/index";
    import {Button} from "$lib/components/ui/button/index";
    import {toast} from "svelte-sonner";
    import {z} from "zod";

    const API_BASE_URL = import.meta.env.VITE_API_BASE_URL;

    const registerSchema = z
        .object({
            name: z
                .string()
                .min(2, "Name must be at least 2 characters")
                .max(50, "Name must be less than 50 characters")
                .regex(
                    /^[a-zA-Z\s]*$/,
                    "First name can only contain letters and spaces",
                ),
            email: z
                .email("Please enter a valid email address")
                .min(5, "Email must be at least 5 characters")
                .max(100, "Email must be less than 100 characters"),
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
            confirmPassword: z
                .string()
                .min(8, "Confirm Password must be 8 - 128 characters long")
                .max(128, "Confirm Password is too long"),
        })
        .refine((data) => data.password === data.confirmPassword, {
            message: "Passwords do not match",
            path: ["confirmPassword"],
        });

    let name = $state("");
    let email = $state("");
    let password = $state("");
    let confirmPassword = $state("");
    let isLoading = $state(false);
    type RegisterField = "name" | "email" | "password" | "confirmPassword";
    type RegisterFormErrors = Partial<
        Record<RegisterField, { message?: string }[]>
    >;
    let fieldErrors = $state<RegisterFormErrors>({});

    async function handleSubmit(e: SubmitEvent) {
        e.preventDefault();

        if (!API_BASE_URL) {
            toast.error("Missing VITE_API_BASE_URL in .env");
            return;
        }

        fieldErrors = {};

        const parsed = registerSchema.safeParse({
            name,
            email,
            password,
            confirmPassword,
        });
        if (!parsed.success) {
            const nextErrors: RegisterFormErrors = {};
            for (const issue of parsed.error.issues) {
                const key = issue.path[0];
                if (
                    key === "name" ||
                    key === "email" ||
                    key === "password" ||
                    key === "confirmPassword"
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
            name = payload.name;
            email = payload.email;

            const res = await fetch(`${API_BASE_URL}/auth/register`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify(payload),
            });

            const data = await res.json().catch(() => ({}));
            const message =
                data?.message ??
                (res.ok ? "Register success" : "Register failed");
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
<div class="flex min-h-svh w-full items-center justify-center p-6 md:p-10">
    <div class="w-full max-w-sm">
        <Card.Root>
            <Card.Header>
                <Card.Title>Create an account</Card.Title>
                <Card.Description>
                    Enter your information below to create your account
                </Card.Description>
            </Card.Header>
            <Card.Content>
                <form onsubmit={handleSubmit}>
                    <Field.Group>
                        <Field.Field>
                            <Field.Label for="name">Full Name</Field.Label>
                            <Input
                                    bind:value={name}
                                    id="name"
                                    name="name"
                                    placeholder="Darcy Jason"
                                    required
                                    type="text"
                            />
                            <Field.Error errors={fieldErrors.name}/>
                        </Field.Field>
                        <Field.Field>
                            <Field.Label for="email">Email</Field.Label>
                            <Input
                                    bind:value={email}
                                    id="email"
                                    name="email"
                                    placeholder="m@example.com"
                                    required
                                    type="email"
                            />
                            <Field.Error errors={fieldErrors.email}/>
                            <Field.Description>
                                We'll use this to contact you. We will not share
                                your email with anyone else.
                            </Field.Description>
                        </Field.Field>
                        <Field.Field>
                            <Field.Label for="password">Password</Field.Label>
                            <Input
                                    bind:value={password}
                                    id="password"
                                    name="password"
                                    required
                                    type="password"
                            />
                            <Field.Error errors={fieldErrors.password}/>
                            <Field.Description
                            >Must be 8 - 128 characters long.
                            </Field.Description
                            >
                        </Field.Field>
                        <Field.Field>
                            <Field.Label for="confirm-password"
                            >Confirm Password
                            </Field.Label
                            >
                            <Input
                                    bind:value={confirmPassword}
                                    id="confirm-password"
                                    name="confirmPassword"
                                    required
                                    type="password"
                            />
                            <Field.Error errors={fieldErrors.confirmPassword}/>
                            <Field.Description
                            >Please confirm your password.
                            </Field.Description
                            >
                        </Field.Field>
                        <Field.Field>
                            <Button disabled={isLoading} type="submit">
                                {isLoading
                                    ? "Creating account..."
                                    : "Create account"}
                            </Button>
                            <Field.Description class="px-6 text-center">
                                Already have an account? <a href="/login"
                            >Sign in</a
                            >
                            </Field.Description>
                        </Field.Field>
                    </Field.Group>
                </form>
            </Card.Content>
        </Card.Root>
    </div>
</div>
