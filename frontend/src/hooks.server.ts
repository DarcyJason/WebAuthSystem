import type { Handle, HandleFetch } from "@sveltejs/kit";
import { PUBLIC_API_BASE_URL } from "$env/static/public";

const API_BASE = PUBLIC_API_BASE_URL.replace(/\/$/, "");
const IS_PROD = !!import.meta.env.PROD;

function extractCookieValueFromSetCookie(setCookieHeader: string | null, cookieName: string): string | null {
  if (!setCookieHeader) return null;
  try {
    const regex = new RegExp(`${cookieName}=([^;\\s]+)`);
    const m = setCookieHeader.match(regex);
    return m ? decodeURIComponent(m[1]) : null;
  } catch {
    return null;
  }
}

export const handle: Handle = async ({ event, resolve }) => {
  event.locals.accessToken = null;
  return resolve(event);
};

export const handleFetch: HandleFetch = async ({ event, request, fetch }) => {
  const url = new URL(request.url);
  const isApiRequest = API_BASE && request.url.startsWith(API_BASE);

  if (!isApiRequest) {
    return fetch(request);
  }

  const refreshEndpointPath = "/api/v1/auth/rotate-refresh-token";
  const isRefreshRequest = request.url.endsWith(`${API_BASE}${refreshEndpointPath}`) ||
    url.pathname.endsWith(refreshEndpointPath) ||
    request.url.endsWith(refreshEndpointPath);

  let accessToken = event.locals.accessToken;
  let modifiedRequest = request;

  if (accessToken) {
    const headers = new Headers(request.headers);
    if (!headers.has("authorization")) {
      headers.set("Authorization", `Bearer ${accessToken}`);
    }
    modifiedRequest = new Request(request, { headers });
  }

  let response = await fetch(modifiedRequest);

  if (response.status !== 401) {
    return response;
  }

  if (isRefreshRequest) {
    return response;
  }

  const refreshToken = event.cookies.get("refresh_token");
  if (!refreshToken) {
    return response;
  }

  try {
    const rotateUrl = `${API_BASE}${refreshEndpointPath}`;
    const rotateHeaders = new Headers();
    rotateHeaders.set("Content-Type", "application/json");
    rotateHeaders.set("Cookie", `refresh_token=${encodeURIComponent(refreshToken)}`);

    const rotateResp = await fetch(rotateUrl, {
      method: "POST",
      headers: rotateHeaders,
    });

    if (!rotateResp.ok) {
      event.cookies.delete("refresh_token", { path: "/" });
      event.locals.accessToken = null;
      return response;
    }

    const newAuthHeader = rotateResp.headers.get("authorization") || rotateResp.headers.get("Authorization");
    let newAccessToken: string | null = null;
    if (newAuthHeader && newAuthHeader.startsWith("Bearer ")) {
      newAccessToken = newAuthHeader.slice("Bearer ".length);
    }

    const setCookieHeader = rotateResp.headers.get("set-cookie");
    const newRefreshToken = extractCookieValueFromSetCookie(setCookieHeader, "refresh_token");

    if (newAccessToken) {
      event.locals.accessToken = newAccessToken;
    }

    if (newRefreshToken) {
      event.cookies.set("refresh_token", newRefreshToken, {
        httpOnly: true,
        path: "/",
        sameSite: "strict",
        secure: IS_PROD,
      });
    }

    if (!newAccessToken) {
      return response;
    }

    const retryHeaders = new Headers(request.headers);
    retryHeaders.set("Authorization", `Bearer ${newAccessToken}`);
    const retryRequest = new Request(request, { headers: retryHeaders });

    return fetch(retryRequest);
  } catch {
    return response;
  }
};
