import type { Breed } from "$lib/enums";
import type { ApiResponse, Litter, Name, SsrFetch } from "$lib/types";

export const API_LITTER = "/api/litter";
export const SSR_LITTER = "http://api:3000/litter";

export async function fetchLitterNames(
    ssr?: SsrFetch,
): Promise<ApiResponse<Name[]>> {
    const options = {
        headers: { "Content-Type": "application/json" },
        method: "GET",
    };

    const res = ssr
        ? await ssr(SSR_LITTER + "/names", options)
        : await fetch(API_LITTER + "/names", options);

    const body = await res.json();
    return res.ok
        ? { res: { type: "data", data: body.data } }
        : { res: { type: "error", error: body.error } };
}

export async function fetchLitter(id: number): Promise<ApiResponse<Litter>> {
    const options = {
        headers: { "Content-Type": "application/json" },
        method: "POST",
        body: JSON.stringify({ id }),
    };

    const res = await fetch(API_LITTER + "/one", options);
    const body = await res.json();
    return res.ok
        ? { res: { type: "data", data: body.data } }
        : { res: { type: "error", error: body.error } };
}

export async function fetchAvaialbleLitters(
    ssr?: SsrFetch,
): Promise<ApiResponse<Litter[]>> {
    const options = {
        headers: { "Content-Type": "application/json" },
        method: "GET",
    };

    const res = ssr
        ? await ssr(SSR_LITTER + "/available", options)
        : await fetch(API_LITTER + "/available", options);

    const body = await res.json();
    return res.ok
        ? { res: { type: "data", data: body.data } }
        : { res: { type: "error", error: body.error } };
}

export async function fetchAvaialbleLittersByBreed(
    breed: Breed,
    ssr?: SsrFetch,
): Promise<ApiResponse<Litter[]>> {
    const options = {
        headers: { "Content-Type": "application/json" },
        method: "POST",
        body: JSON.stringify({ breed }),
    };

    const res = ssr
        ? await ssr(SSR_LITTER + "/breed", options)
        : await fetch(API_LITTER + "/breed", options);

    const body = await res.json();
    return res.ok
        ? { res: { type: "data", data: body.data } }
        : { res: { type: "error", error: body.error } };
}

export async function deleteLitter(
    id: number,
    jwt: string,
): Promise<ApiResponse<never>> {
    const options = {
        headers: { "Content-Type": "application/json", Authorization: jwt },
        method: "DELETE",
        body: JSON.stringify({ id }),
    };

    const res = await fetch(API_LITTER + "/delete", options);
    const body = await res.json();
    return res.ok
        ? { res: { type: "success" } }
        : { res: { type: "error", error: body.error } };
}
