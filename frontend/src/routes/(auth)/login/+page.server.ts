import { loginSchema } from "$lib/schema/login";
import { fail, type Actions } from "@sveltejs/kit";
import { superValidate } from "sveltekit-superforms";
import { zod4 } from "sveltekit-superforms/adapters";
import { PUBLIC_API_BASE_URL } from "$env/static/public";

const API_BASE_URL = PUBLIC_API_BASE_URL.replace(/\/$/, "");

export const load = async (event: any) => {
  const form = await superValidate(event, zod4(loginSchema));
  return { form };
};

export const actions: Actions = {
  default: async (event) => {
    const form = await superValidate(event, zod4(loginSchema));
    if (!form.valid) {
      return fail(400, { form });
    }

    let response: Response;
    try {
      response = await event.fetch(`${API_BASE_URL}/api/v1/auth/login`, {
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
          message: resultData.message ?? "Login failed",
        },
      });
    }

    const authHeader =
      response.headers.get("authorization") ??
      response.headers.get("Authorization");
    let accessToken: string | null = null;
    if (authHeader?.startsWith("Bearer ")) {
      accessToken = authHeader.slice("Bearer ".length);
    }

    const setCookieHeader = response.headers.get("set-cookie");
    if (setCookieHeader) {
      const m = /refresh_token=([^;]+)/.exec(setCookieHeader);
      if (m) {
        event.cookies.set("refresh_token", m[1], {
          httpOnly: true,
          path: "/",
          sameSite: "strict",
          secure: !!import.meta.env.PROD,
          maxAge: 60 * 60 * 24 * 7,
        });
      }
    }

    return {
      form,
      result: {
        status: 200,
        message: "Login successful",
        accessToken,
      },
    };
  },
};
