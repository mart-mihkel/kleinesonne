import type { ApiResponse, Dog, Family, SsrFetch } from "$lib/types";
import { SSR_DOG } from "$lib/api/dog";
import { redis } from "./redis";

export async function fetchFamily(name: string): Promise<ApiResponse<Family>> {
    const resolver = async (name: string, depth = 4): Promise<Family> => {
        if (depth === 0) {
            return { name };
        }

        const res = await redis.get(name);
        if (res === null) {
            return { name };
        }

        type Parents = { father: string; mother: string };
        const parents: Parents = JSON.parse(res);
        const fp = resolver(parents.father, depth - 1);
        const mp = resolver(parents.mother, depth - 1);

        const father = await fp;
        const mother = await mp;

        return { name, father, mother };
    };

    const family = await resolver(name);
    return { res: { type: "data", data: family } };
}

export async function updateFamily(
    name: string,
    father: string | undefined,
    mother: string | undefined,
): Promise<ApiResponse<never>> {
    const parents = { father, mother };
    const res = await redis.set(name, JSON.stringify(parents));

    if (res === null) {
        return { res: { type: "error", error: "Update family failed" } };
    }

    return { res: { type: "success" } };
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
