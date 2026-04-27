import { resetPasswordSchema } from "$lib/schema/reset-password";
import { fail, type Actions } from "@sveltejs/kit";
import { superValidate } from "sveltekit-superforms";
import { zod4 } from "sveltekit-superforms/adapters";

const API_BASE_URL = (
  import.meta.env.PUBLIC_API_BASE_URL ??
  import.meta.env.API_BASE_URL ??
  ""
).replace(/\/$/, "");

export const load = async ({ url }) => {
  const token = url.searchParams.get("token") ?? "";
  const form = await superValidate(
    { token, newPassword: "", confirmPassword: "" },
    zod4(resetPasswordSchema),
  );
  return { form, hasToken: token.length > 0 };
};

export const actions: Actions = {
  default: async (event) => {
    const form = await superValidate(event, zod4(resetPasswordSchema));
    if (!form.valid) {
      return fail(400, { form });
    }

    let response: Response;
    try {
      response = await event.fetch(
        `${API_BASE_URL}/api/v1/auth/reset-password`,
        {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({
            token: form.data.token,
            newPassword: form.data.newPassword,
            confirmPassword: form.data.confirmPassword,
          }),
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
          message: resultData.message ?? "Password reset failed",
        },
      });
    }

    return {
      form,
      result: {
        status: 200,
        message:
          resultData.message ??
          "Password reset successfully! You can now sign in with your new password.",
      },
    };
  },
};
