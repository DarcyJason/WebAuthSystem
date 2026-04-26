import { loginSchema } from "$lib/schema/login";
import { fail, type Actions, redirect } from "@sveltejs/kit";
import { superValidate } from "sveltekit-superforms";
import { zod4 } from "sveltekit-superforms/adapters";

const API_BASE_URL =
  import.meta.env.PUBLIC_API_BASE_URL ?? import.meta.env.API_BASE_URL;

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
      const response = await event.fetch(`${API_BASE_URL}/api/v1/auth/login`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(form.data),
      });

      const resultData = await response.json().catch(() => ({}));

      if (!response.ok) {
        return fail(response.status, {
          form,
          result: {
            status: response.status,
            message: resultData.message || "Login failed",
          },
        });
      }

      // Try to read Authorization header (Access Token) and Set-Cookie (Refresh Token)
      const authHeader =
        response.headers.get("authorization") ||
        response.headers.get("Authorization");
      if (authHeader && authHeader.startsWith("Bearer ")) {
        const accessToken = authHeader.slice("Bearer ".length);
        event.cookies.set("access_token", accessToken, {
          httpOnly: true,
          path: "/",
          sameSite: "lax",
          secure: process.env.NODE_ENV === "production",
          maxAge: 60 * 60, // 1 hour, adjust to match backend access token lifetime
        });
      }

      // If backend returned Set-Cookie header with refresh_token, parse and set it as httpOnly cookie.
      const setCookieHeader = response.headers.get("set-cookie");
      if (setCookieHeader) {
        const m = /refresh_token=([^;]+)/.exec(setCookieHeader);
        if (m) {
          event.cookies.set("refresh_token", m[1], {
            httpOnly: true,
            path: "/",
            sameSite: "strict",
            secure: process.env.NODE_ENV === "production",
            maxAge: 60 * 60 * 24 * 7, // 7 days, keep aligned with backend
          });
        }
      }

      // Successful login: redirect to dashboard
      throw redirect(303, "/dashboard");
    } catch (err) {
      return fail(500, {
        form,
        result: { status: 500, message: "Server unreachable" },
      });
    }
  },
};
