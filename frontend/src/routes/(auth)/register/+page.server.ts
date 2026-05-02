import { registerSchema } from "$lib/schema/register";
import { fail, redirect, type Actions } from "@sveltejs/kit";
import { superValidate } from "sveltekit-superforms";
import { zod4 } from "sveltekit-superforms/adapters";
import { PUBLIC_API_BASE_URL } from "$env/static/public";

const API_BASE_URL = PUBLIC_API_BASE_URL.replace(/\/$/, "");

export const load = async (event: any) => {
  const form = await superValidate(event, zod4(registerSchema));
  return { form };
};

export const actions: Actions = {
  default: async (event) => {
    const form = await superValidate(event, zod4(registerSchema));
    if (!form.valid) {
      return fail(400, { form });
    }

    let response: Response;
    try {
      response = await event.fetch(`${API_BASE_URL}/api/v1/auth/register`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(form.data),
      });
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
          message: resultData.message ?? "Registration failed",
        },
      });
    }

    const email = encodeURIComponent(form.data.email);
    throw redirect(303, `/verify?email=${email}`);
  },
};
