import { type Image } from "$lib/types";

export const API_UPLOADS = "http://127.0.0.1:3000/uploads";

export async function uploadImages(
    images: Image[],
    jwt: string,
): Promise<boolean> {
    const options = {
        headers: { "Content-Type": "application/json", Authorization: jwt },
        method: "POST",
        body: JSON.stringify(images),
    };

    const res = await fetch(API_UPLOADS, options);
    return res.ok;
}

export async function deleteImage(
    id: number,
    jwt: string,
    endpoint: string,
    image?: string,
): Promise<boolean> {
    const options = {
        headers: { "Content-Type": "application/json", Authorization: jwt },
        method: "DELETE",
        body: JSON.stringify({ id, image }),
    };

    const res = await fetch(endpoint, options);
    return res.ok;
}
