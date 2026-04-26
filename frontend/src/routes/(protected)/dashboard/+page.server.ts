import type { ServerLoad } from "@sveltejs/kit";
import { redirect } from "@sveltejs/kit";

const API = import.meta.env.PUBLIC_API_BASE_URL;

/**
 * Server-side load for the protected dashboard.
 * - Calls backend /api/v1/me using server-side fetch so cookies are available.
 * - If unauthenticated (401) or any failure, redirect to /login.
 */
export const load: ServerLoad = async ({ fetch }) => {
  // Call the backend to validate current session / access token.
  // `handleFetch` will inject Authorization header from cookie for server-side fetches.
  const res = await fetch(`${API}/api/v1/me`);

  if (res.status === 401) {
    // Not authenticated -> redirect to login
    throw redirect(303, "/login");
  }

  if (!res.ok) {
    // For other errors, treat as unauthenticated to be safe.
    throw redirect(303, "/login");
  }

  const payload = await res.json().catch(() => null);

  return {
    user: payload?.data ?? null,
  };
};
