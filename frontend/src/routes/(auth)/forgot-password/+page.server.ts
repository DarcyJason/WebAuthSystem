import { forgotPasswordSchema } from "$lib/schema/forgot-password";
import { fail, type Actions } from "@sveltejs/kit";
import { superValidate } from "sveltekit-superforms";
import { zod4 } from "sveltekit-superforms/adapters";
import { PUBLIC_API_BASE_URL } from "$env/static/public";

const API_BASE_URL = PUBLIC_API_BASE_URL.replace(/\/$/, "");

export const load = async (event: any) => {
  const form = await superValidate(event, zod4(forgotPasswordSchema));
  return { form };
};

export const actions: Actions = {
  default: async (event) => {
    const form = await superValidate(event, zod4(forgotPasswordSchema));
    if (!form.valid) {
      return fail(400, { form });
    }

    let response: Response;
    try {
      response = await event.fetch(
        `${API_BASE_URL}/api/v1/auth/forgot-password`,
        {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify(form.data),
        },
      );
    } catch {
      return fail(500, {
        form,
        result: { status: 500, message: "Server unreachable" },
      });
    }

    const resultData = await response.json().catch(() => ({}));

    if (!response.ok) {
      return fail(response.status, {
        form,
        result: {
          status: response.status,
          message: resultData.message ?? "Failed to send reset email",
        },
      });
    }

    return {
      form,
      result: {
        status: 200,
        message:
          resultData.message ??
          "If an account exists with that email, you'll receive a reset link shortly.",
      },
    };
  },
};
