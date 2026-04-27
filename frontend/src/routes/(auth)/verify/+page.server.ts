import { fail, type Actions } from "@sveltejs/kit";

const API_BASE_URL = (
  import.meta.env.PUBLIC_API_BASE_URL ??
  import.meta.env.API_BASE_URL ??
  ""
).replace(/\/$/, "");

export const load = async ({ url, fetch }) => {
  const token = url.searchParams.get("token");

  if (!token) {
    return { verified: null as null, message: null as null };
  }

  try {
    const response = await fetch(`${API_BASE_URL}/api/v1/auth/verify`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ token }),
    });

    const data = await response.json().catch(() => ({}));

    if (response.ok) {
      return {
        verified: true as const,
        message: (data.message as string) ?? "Email verified successfully!",
      };
    } else {
      return {
        verified: false as const,
        message:
          (data.message as string) ??
          "Verification failed. The link may be expired or invalid.",
      };
    }
  } catch {
    return {
      verified: false as const,
      message: "Server unreachable. Please try again later.",
    };
  }
};

export const actions: Actions = {
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
