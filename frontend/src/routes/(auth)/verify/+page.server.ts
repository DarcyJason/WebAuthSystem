import { fail, redirect, type Actions } from "@sveltejs/kit";
import { PUBLIC_API_BASE_URL } from "$env/static/public";

const API_BASE_URL = PUBLIC_API_BASE_URL.replace(/\/$/, "");

export const load = async ({ url }: { url: URL }) => {
  const email = url.searchParams.get("email") ?? "";
  return { email };
};

export const actions: Actions = {
  verify: async (event) => {
    const formData = await event.request.formData();
    const token = String(formData.get("token") ?? "").trim();

    if (!token) {
      return fail(400, { verifyError: "Please enter a verification token." });
    }

    let response: Response;
    try {
      response = await event.fetch(`${API_BASE_URL}/api/v1/auth/verify`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ token }),
      });
    } catch {
      return fail(500, {
        verifyError: "Server unreachable. Please try again later.",
      });
    }

    const data = await response.json().catch(() => ({}));
    if (!response.ok) {
      return fail(response.status, {
        verifyError: data.message ?? "Verification failed. Token may be invalid or expired.",
      });
    }

    throw redirect(303, "/login");
  },
  resend: async (event) => {
    const formData = await event.request.formData();
    const email = formData.get("email") as string;

    if (!email || !email.includes("@")) {
      return fail(400, { error: "Please enter a valid email address" });
    }

    let response: Response;
    try {
      response = await event.fetch(
        `${API_BASE_URL}/api/v1/auth/resend-verification`,
        {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ email }),
        },
      );
    } catch {
      return fail(500, { error: "Server unreachable. Please try again later." });
    }

    const data = await response.json().catch(() => ({}));

    if (!response.ok) {
      return fail(response.status, {
        error: data.message ?? "Failed to resend verification email",
      });
    }

    return {
      success: true,
      message:
        data.message ?? "Verification email sent! Please check your inbox.",
    };
  },
};
