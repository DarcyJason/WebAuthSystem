import type { Actions, ServerLoad } from "@sveltejs/kit";
import { redirect } from "@sveltejs/kit";
import { PUBLIC_API_BASE_URL } from "$env/static/public";

const API_BASE = PUBLIC_API_BASE_URL.replace(/\/$/, "");

export const load: ServerLoad = async ({ fetch, cookies }) => {
  try {
    await fetch(`${API_BASE}/api/v1/auth/logout`, {
      method: "POST",
    });
  } catch {
  } finally {
    try {
      cookies.delete("refresh_token", { path: "/" });
    } catch {}
  }

  throw redirect(303, "/login");
};

export const actions: Actions = {
  default: async (event) => {
    try {
      await event.fetch(`${API_BASE}/api/v1/auth/logout`, {
        method: "POST",
      });
    } catch {} finally {
      try {
        event.cookies.delete("refresh_token", { path: "/" });
      } catch {}
    }

    throw redirect(303, "/login");
  },
};
