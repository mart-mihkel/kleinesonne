import { Gender, Breed } from "$lib/enums";
import type { ApiResponse, Dog, Family, Name, SsrFetch } from "$lib/types";

export const API_DOG = "/api/dog";
const SSR_DOG = "http://api:3000/dog";

export async function fetchFamily(
    ssr: SsrFetch,
    name: string,
): Promise<ApiResponse<Family>> {
    // TODO: graphdb
    console.log("fetch family ", ssr, name);
    return { res: { type: "success" } };
}

export async function fetchDogNames(): Promise<ApiResponse<Name[]>> {
    const options = {
        headers: { "Content-Type": "application/json" },
        method: "GET",
    };

    const res = await fetch(API_DOG + "/names", options);
    const body = await res.json();
    return res.ok
        ? { res: { type: "data", data: body.data } }
        : { res: { type: "error", error: body.error } };
}

export async function fetchDog(
    id: number,
    ssr?: SsrFetch,
): Promise<ApiResponse<Dog>> {
    const options = {
        headers: { "Content-Type": "application/json" },
        method: "POST",
        body: JSON.stringify({ id }),
    };

    const res = ssr
        ? await ssr(SSR_DOG + "/one", options)
        : await fetch(API_DOG + "/one", options);

    const body = await res.json();
    return res.ok
        ? { res: { type: "data", data: body.data } }
        : { res: { type: "error", error: body.error } };
}

export async function fetchAliveDogs(
    breed: Breed,
    gender: Gender,
    ssr?: SsrFetch,
): Promise<ApiResponse<Dog[]>> {
    const options = {
        headers: { "Content-Type": "application/json" },
        method: "POST",
        body: JSON.stringify({
            breed,
            gender,
        }),
    };

    const res = ssr
        ? await ssr(SSR_DOG + "/alive", options)
        : await fetch(API_DOG + "/alive", options);

    const body = await res.json();
    return res.ok
        ? { res: { type: "data", data: body.data } }
        : { res: { type: "error", error: body.error } };
}

export async function fetchRetiredDogs(
    breed: Breed,
    ssr?: SsrFetch,
): Promise<ApiResponse<Dog[]>> {
    const options = {
        headers: { "Content-Type": "application/json" },
        method: "POST",
        body: JSON.stringify({
            breed,
        }),
    };

    const res = ssr
        ? await ssr(SSR_DOG + "/retired", options)
        : await fetch(API_DOG + "/retired", options);

    const body = await res.json();
    return res.ok
        ? { res: { type: "data", data: body.data } }
        : { res: { type: "error", error: body.error } };
}

export async function uploadDog(
    fetch: SsrFetch,
    dog: Dog,
    jwt: string,
): Promise<ApiResponse<number>> {
    const options = {
        headers: { "Content-Type": "application/json", Authorization: jwt },
        method: "PUT",
        body: JSON.stringify(dog),
    };

    const res = await fetch(SSR_DOG + "/new", options);
    const body = await res.json();
    return res.ok
        ? { res: { type: "data", data: body.data } }
        : { res: { type: "error", error: body.error } };
}

export async function updateDog(
    fetch: SsrFetch,
    dog: Dog,
    jwt: string,
): Promise<ApiResponse<never>> {
    const options = {
        headers: { "Content-Type": "application/json", Authorization: jwt },
        method: "PUT",
        body: JSON.stringify(dog),
    };

    const res = await fetch(SSR_DOG + "/update", options);
    const body = await res.json();
    return res.ok
        ? { res: { type: "success" } }
        : { res: { type: "error", error: body.error } };
}

export async function deleteDog(
    id: number,
    jwt: string,
): Promise<ApiResponse<never>> {
    const options = {
        headers: { "Content-Type": "application/json", Authorization: jwt },
        method: "DELETE",
        body: JSON.stringify({ id }),
    };

    const res = await fetch(API_DOG + "/delete", options);
    const body = await res.json();
    return res.ok
        ? { res: { type: "success" } }
        : { res: { type: "error", error: body.error } };
}
