import type { ApiResponse, SsrFetch } from "$lib/types";

export const API_AUTH = "/api/auth";

export async function authenticate(
    fetch: SsrFetch,
    jwt: string,
): Promise<ApiResponse<never>> {
    const options = {
        headers: {
            "Content-Type": "application/json",
            Authorization: jwt,
        },
        method: "GET",
    };

    const res = await fetch(API_AUTH, options);
    const body = await res.json();
    return res.ok
        ? { res: { type: "success" } }
        : { res: { type: "error", error: body.error } };
}

export async function login(
    fetch: SsrFetch,
    user: string,
    secret: string,
): Promise<ApiResponse<string>> {
    const options = {
        headers: { "Content-Type": "application/json" },
        method: "POST",
        body: JSON.stringify({ user, secret }),
    };

    const res = await fetch(API_AUTH, options);
    const body = await res.json();
    return res.ok
        ? { res: { type: "token", token: body.token } }
        : { res: { type: "error", error: body.error } };
}
