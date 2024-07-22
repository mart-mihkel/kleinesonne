import type { ApiResponse } from "$lib/types";

export const API_UPLOADS = "/api/uploads";

export async function deleteImage(
    id: number,
    jwt: string,
    endpoint: string,
    image?: string,
): Promise<ApiResponse<never>> {
    const options = {
        headers: { "Content-Type": "application/json", Authorization: jwt },
        method: "DELETE",
        body: JSON.stringify({ id, image }),
    };

    const res = await fetch(endpoint, options);
    const body = await res.json();
    return res.ok
        ? { res: { type: "success" } }
        : { res: { type: "error", error: body.error } };
}
