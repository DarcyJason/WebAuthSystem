import { loginSchema } from "$lib/schema/login";
import { fail, type Actions } from "@sveltejs/kit";
import { superValidate } from "sveltekit-superforms";
import { zod4 } from "sveltekit-superforms/adapters";

const { API_BASE_URL } = import.meta.env;

export const load = async (event) => {
  const form = await superValidate(event, zod4(loginSchema));
  return {
    form,
  };
};

export const actions: Actions = {
  default: async (event) => {
    const form = await superValidate(event, zod4(loginSchema));
    if (!form.valid) {
      return fail(400, { form });
    }
    try {
      const response = await event.fetch(`${API_BASE_URL}/auth/login`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(form.data),
      });
      const resultData = await response.json();
      if (!response.ok) {
        return fail(response.status, {
          form,
          result: {
            status: response.status,
            message: resultData.message || "Login failed",
          },
        });
      }
      return {
        form,
        result: {
          status: 200,
          message: resultData.message || "Success!",
        },
      };
    } catch (err) {
      return fail(500, {
        form,
        result: { status: 500, message: "Server unreachable" },
      });
    }
  },
};
