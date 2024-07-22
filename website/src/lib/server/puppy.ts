import type { ApiResponse, Puppy, SsrFetch } from "$lib/types";
import { SSR_PUPPY } from "$lib/api/puppy";

export async function uploadPuppy(
    fetch: SsrFetch,
    puppy: Puppy,
    jwt: string,
): Promise<ApiResponse<number>> {
    const options = {
        headers: { "Content-Type": "application/json", Authorization: jwt },
        method: "PUT",
        body: JSON.stringify(puppy),
    };

    const res = await fetch(SSR_PUPPY + "/new", options);
    const body = await res.json();
    return res.ok
        ? { res: { type: "data", data: body.data } }
        : { res: { type: "error", error: body.error } };
}

export async function updatePuppy(
    fetch: SsrFetch,
    puppy: Puppy,
    jwt: string,
): Promise<ApiResponse<never>> {
    const options = {
        headers: { "Content-Type": "application/json", Authorization: jwt },
        method: "PUT",
        body: JSON.stringify(puppy),
    };

    const res = await fetch(SSR_PUPPY + "/update", options);
    const body = await res.json();
    return res.ok
        ? { res: { type: "success" } }
        : { res: { type: "error", error: body.error } };
}
