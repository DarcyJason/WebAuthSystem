import { changePasswordSchema } from "$lib/schema/change-password";
import { fail, redirect, type Actions, type ServerLoad } from "@sveltejs/kit";
import { superValidate } from "sveltekit-superforms";
import { zod4 } from "sveltekit-superforms/adapters";
import { PUBLIC_API_BASE_URL } from "$env/static/public";

const API_BASE_URL = PUBLIC_API_BASE_URL.replace(/\/$/, "");

export const load: ServerLoad = async (event) => {
  const res = await event.fetch(`${API_BASE_URL}/api/v1/protected/me`);
  if (!res.ok) {
    throw redirect(303, "/login");
  }

  const form = await superValidate(event, zod4(changePasswordSchema));
  return { form };
};

export const actions: Actions = {
  default: async (event) => {
    const form = await superValidate(event, zod4(changePasswordSchema));
    if (!form.valid) {
      return fail(400, { form });
    }

    let response: Response;
    try {
      response = await event.fetch(`${API_BASE_URL}/api/v1/protected/change-password`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          currentPassword: form.data.currentPassword,
          newPassword: form.data.newPassword,
          confirmPassword: form.data.confirmPassword,
        }),
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
          message: resultData.message ?? "Failed to change password",
        },
      });
    }

    return {
      form,
      result: {
        status: 200,
        message: resultData.message ?? "Password changed successfully!",
      },
    };
  },
};
