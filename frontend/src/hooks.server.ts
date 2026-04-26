/**
 * frontend/src/hooks.server.ts
 *
 * SvelteKit server hook to:
 *  - inject Authorization header from httpOnly `access_token` cookie for requests to the API backend,
 *  - on 401 responses attempt to refresh access token using the httpOnly `refresh_token` cookie by calling
 *    the backend `/api/v1/auth/rotate-refresh-token` endpoint,
 *  - if refresh succeeds, update cookies (`access_token`, `refresh_token`) and retry the original request once.
 *
 * Notes:
 *  - This code assumes the backend expose endpoints under PUBLIC_API_BASE_URL (vite env var).
 *  - It intentionally scopes header injection & refresh logic to requests targeting the configured API base.
 *  - Cookie options use `import.meta.env.PROD` to decide `secure` flag for production builds.
 */

import type { HandleFetch } from "@sveltejs/kit";

const API_BASE = (import.meta.env.PUBLIC_API_BASE_URL ?? "").replace(/\/$/, ""); // no trailing slash
const IS_PROD = !!import.meta.env.PROD;

/**
 * Helper: extract cookie value from a Set-Cookie header string.
 * Returns the value for cookieName or null.
 */
function extractCookieValueFromSetCookie(setCookieHeader: string | null, cookieName: string): string | null {
  if (!setCookieHeader) return null;
  try {
    // There may be multiple Set-Cookie headers concatenated by the fetch implementation into a single string.
    // We'll search for the cookieName followed by '=' and capture until ';' or end.
    const regex = new RegExp(`${cookieName}=([^;\\s]+)`);
    const m = setCookieHeader.match(regex);
    return m ? decodeURIComponent(m[1]) : null;
  } catch {
    return null;
  }
}

export const handleFetch: HandleFetch = async ({ event, request, fetch }) => {
  // Only operate on requests that target our API backend
  const url = new URL(request.url);
  const isApiRequest = API_BASE && request.url.startsWith(API_BASE);

  // If not API request, let it pass through unchanged
  if (!isApiRequest) {
    return fetch(request);
  }

  // Avoid attempting refresh for refresh endpoint itself to prevent recursion
  const refreshEndpointPath = "/api/v1/auth/rotate-refresh-token";
  if (url.pathname.endsWith(refreshEndpointPath.replace(API_BASE, "")) || request.url.endsWith(refreshEndpointPath)) {
    // For the refresh endpoint, we still want to include Authorization if available, but avoid retry loops.
    // Continue to perform a fetch with cookie injection handled below.
  }

  // 1. Inject Authorization header from access_token cookie (if exists)
  const accessToken = event.cookies.get("access_token");
  let modifiedRequest = request;

  if (accessToken) {
    const headers = new Headers(request.headers);
    // Do not overwrite an existing Authorization header if it is already present
    if (!headers.has("authorization")) {
      headers.set("Authorization", `Bearer ${accessToken}`);
    }
    modifiedRequest = new Request(request, { headers });
  }

  // 2. Perform the request
  let response = await fetch(modifiedRequest);

  // 3. If not 401, return immediately
  if (response.status !== 401) {
    return response;
  }

  // 4. On 401, attempt token refresh (only for API requests and not for refresh endpoint itself)
  // Avoid refreshing if the request was the refresh endpoint itself to prevent infinite loop.
  const isRefreshRequest = request.url.endsWith(`${API_BASE}/api/v1/auth/rotate-refresh-token`) ||
    url.pathname.endsWith("/api/v1/auth/rotate-refresh-token") ||
    request.url.endsWith("/api/v1/auth/rotate-refresh-token");

  if (!isApiRequest || isRefreshRequest) {
    // Can't refresh here: return the original 401 response
    return response;
  }

  const refreshToken = event.cookies.get("refresh_token");
  if (!refreshToken) {
    // No refresh token available, nothing we can do
    return response;
  }

  try {
    // Call rotate-refresh-token endpoint on backend, include refresh_token cookie in Cookie header.
    const rotateUrl = `${API_BASE}/api/v1/auth/rotate-refresh-token`;
    const rotateHeaders = new Headers();
    rotateHeaders.set("Content-Type", "application/json");
    // Include the refresh_token cookie value in the Cookie header for the backend to read
    rotateHeaders.set("Cookie", `refresh_token=${encodeURIComponent(refreshToken)}`);

    const rotateResp = await fetch(rotateUrl, {
      method: "POST",
      headers: rotateHeaders,
    });

    if (!rotateResp.ok) {
      // Refresh failed (expired/invalid refresh token). Clear local cookies to force re-login.
      event.cookies.delete("access_token", { path: "/" });
      event.cookies.delete("refresh_token", { path: "/" });
      return response;
    }

    // Parse new Authorization header from backend response
    const newAuthHeader = rotateResp.headers.get("authorization") || rotateResp.headers.get("Authorization");
    let newAccessToken: string | null = null;
    if (newAuthHeader && newAuthHeader.startsWith("Bearer ")) {
      newAccessToken = newAuthHeader.slice("Bearer ".length);
    }

    // Parse Set-Cookie from refresh response (may contain new refresh_token)
    const setCookieHeader = rotateResp.headers.get("set-cookie");
    const newRefreshToken = extractCookieValueFromSetCookie(setCookieHeader, "refresh_token");

    // Persist tokens to browser cookies (httpOnly) so subsequent requests carry them.
    if (newAccessToken) {
      // set access_token cookie (short lifetime). Adjust maxAge as appropriate (seconds).
      event.cookies.set("access_token", newAccessToken, {
        httpOnly: true,
        path: "/",
        sameSite: "lax",
        secure: IS_PROD,
        maxAge: 60 * 15, // 15 minutes as an example — tune to your access token lifetime
      });
    }

    if (newRefreshToken) {
      // set refresh_token cookie (longer lifetime). Use SameSite=Strict in production if appropriate.
      event.cookies.set("refresh_token", newRefreshToken, {
        httpOnly: true,
        path: "/",
        sameSite: "strict",
        secure: IS_PROD,
        // maxAge not set here; backend controls expiry semantics. You may set one if desired.
      });
    }

    // 5. Retry the original request once with the new access token injected
    const retryAccessToken = newAccessToken ?? accessToken;
    if (!retryAccessToken) {
      // No token available to retry with
      return response;
    }

    const retryHeaders = new Headers(request.headers);
    retryHeaders.set("Authorization", `Bearer ${retryAccessToken}`);
    const retryRequest = new Request(request, { headers: retryHeaders });

    const retryResponse = await fetch(retryRequest);
    // If retryResponse is still 401, propagate that to caller (and let higher-level logic handle redirect to login).
    return retryResponse;
  } catch (err) {
    // On error during refresh process, clear cookies for safety and return original 401 response
    try {
      event.cookies.delete("access_token", { path: "/" });
      event.cookies.delete("refresh_token", { path: "/" });
    } catch {
      // ignore
    }
    return response;
  }
};
