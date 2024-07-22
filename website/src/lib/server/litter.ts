import type { ApiResponse, Litter, SsrFetch } from "$lib/types";
import { SSR_LITTER } from "$lib/api/litter";

export async function uploadLitter(
    fetch: SsrFetch,
    litter: Litter,
    jwt: string,
): Promise<ApiResponse<number>> {
    const options = {
        headers: { "Content-Type": "application/json", Authorization: jwt },
        method: "PUT",
        body: JSON.stringify(litter),
    };

    const res = await fetch(SSR_LITTER + "/new", options);
    const body = await res.json();
    return res.ok
        ? { res: { type: "data", data: body.data } }
        : { res: { type: "error", error: body.error } };
}

export async function updateLitter(
    fetch: SsrFetch,
    litter: Litter,
    jwt: string,
): Promise<ApiResponse<never>> {
    const options = {
        headers: { "Content-Type": "application/json", Authorization: jwt },
        method: "PUT",
        body: JSON.stringify(litter),
    };

    const res = await fetch(SSR_LITTER + "/update", options);
    const body = await res.json();
    return res.ok
        ? { res: { type: "success" } }
        : { res: { type: "error", error: body.error } };
}
