import type { Actions, ServerLoad } from "@sveltejs/kit";
import { redirect } from "@sveltejs/kit";

const API_BASE =
  (import.meta.env.PUBLIC_API_BASE_URL as string) ??
  (import.meta.env.API_BASE_URL as string) ??
  "";

/**
 * When the user navigates to /logout we perform server-side logout:
 *  - call backend `/api/v1/auth/logout` (server-side fetch so cookies are available)
 *  - clear local cookies (`access_token`, `refresh_token`)
 *  - redirect to /login
 *
 * Notes:
 *  - This relies on the handleFetch hook (or server-side behavior) to inject Authorization
 *    header from `access_token` cookie if required by the backend.
 *  - We attempt the backend call but always clear local cookies to ensure the client is logged out.
 */

export const load: ServerLoad = async ({ fetch, cookies }) => {
  try {
    // Call backend logout endpoint. Server-side fetch will include cookies and/or injected Authorization.
    await fetch(`${API_BASE}/api/v1/auth/logout`, {
      method: "POST",
    });
  } catch (err) {
    // ignore network errors; proceed to clear cookies and redirect anyway
  } finally {
    // Clear cookies locally (ensure path matches how they were set)
    try {
      cookies.delete("access_token", { path: "/" });
      cookies.delete("refresh_token", { path: "/" });
    } catch {
      // ignore
    }
  }

  // Redirect to login page
  throw redirect(303, "/login");
};

// Also expose an action in case a POST form triggers logout (form POST or button)
export const actions: Actions = {
  default: async (event) => {
    try {
      await event.fetch(`${API_BASE}/api/v1/auth/logout`, {
        method: "POST",
      });
    } catch {
      // ignore
    } finally {
      try {
        event.cookies.delete("access_token", { path: "/" });
        event.cookies.delete("refresh_token", { path: "/" });
      } catch {
        // ignore
      }
    }

    throw redirect(303, "/login");
  },
};
