import type { ApiResponse, SsrFetch, Image } from "$lib/types";

const SSR_UPLOADS = "http://api:3000/uploads";

export async function uploadImages(
    fetch: SsrFetch,
    images: Image[],
    jwt: string,
): Promise<ApiResponse<never>> {
    if (images.length === 0) {
        return { res: { type: "success" } };
    }

    const options = {
        headers: { "Content-Type": "application/json", Authorization: jwt },
        method: "POST",
        body: JSON.stringify(images),
    };

    const res = await fetch(SSR_UPLOADS, options);
    const body = await res.json();
    return res.ok
        ? { res: { type: "success" } }
        : { res: { type: "error", error: body.error } };
}
