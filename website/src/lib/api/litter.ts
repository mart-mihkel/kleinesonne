import { Breed, type Id, type Litter, type Name } from "$lib/types";

const API_LITTER = "http://127.0.0.1:3000/litter";

export async function fetchLitterNames(): Promise<Name[]> {
    const options = {
        headers: { "Content-Type": "application/json" },
        method: "GET",
    };

    const res = await fetch(API_LITTER + "/names", options);
    const body: Name[] = await res.json();
    return body;
}

export async function fetchLitter(id: number): Promise<Litter> {
    const options = {
        headers: { "Content-Type": "application/json" },
        method: "POST",
        body: JSON.stringify({ id }),
    };

    const res = await fetch(API_LITTER + "/one", options);
    const body: Litter = await res.json();
    return body;
}

export async function fetchAvaialbleLitters(): Promise<Litter[]> {
    const options = {
        headers: { "Content-Type": "application/json" },
        method: "GET",
    };

    const res = await fetch(API_LITTER + "/available", options);
    const body: Litter[] = await res.json();
    return body;
}

export async function fetchAvaialbleLittersByBreed(
    breed: Breed,
): Promise<Litter[]> {
    const options = {
        headers: { "Content-Type": "application/json" },
        method: "POST",
        body: JSON.stringify({ breed }),
    };

    const res = await fetch(API_LITTER + "/breed", options);
    const body: Litter[] = await res.json();
    return body;
}

export async function uploadLitter(litter: Litter): Promise<Id> {
    const options = {
        headers: { "Content-Type": "application/json" },
        method: "PUT",
        body: JSON.stringify({ litter }),
    };

    const res = await fetch(API_LITTER + "/new", options);
    const body: Id = await res.json();
    return body;
}

export async function updateLitter(litter: Litter): Promise<boolean> {
    const options = {
        headers: { "Content-Type": "application/json" },
        method: "PUT",
        body: JSON.stringify({ litter }),
    };

    const res = await fetch(API_LITTER + "/update", options);
    return res.ok;
}

export async function deleteLitter(id: number): Promise<boolean> {
    const options = {
        headers: { "Content-Type": "application/json" },
        method: "DELETE",
        body: JSON.stringify({ id }),
    };

    const res = await fetch(API_LITTER + "/delete", options);
    return res.ok;
}
