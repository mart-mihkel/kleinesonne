import { Breed, type Id, type Litter, type Name } from "$lib/types";
import { API, JSON_HEADERS } from ".";

const API_LITTER = `${API}/litter`;

export async function fetchLitterNames(): Promise<Name[]> {
    const options = {
        headers: JSON_HEADERS,
        method: "GET",
    };

    const res = await fetch(API_LITTER + "/names", options);
    const body: Name[] = await res.json();
    return body;
}

export async function fetchLitter(id: number): Promise<Litter[]> {
    const options = {
        headers: JSON_HEADERS,
        method: "POST",
        body: JSON.stringify({ id }),
    };

    const res = await fetch(API_LITTER + "/one", options);
    const body: Litter[] = await res.json();
    return body;
}

export async function fetchAvaialbleLitters(breed: Breed): Promise<Litter[]> {
    const options = {
        headers: JSON_HEADERS,
        method: "POST",
        body: JSON.stringify({ breed }),
    };

    const res = await fetch(API_LITTER + "/breed", options);
    const body: Litter[] = await res.json();
    return body;
}

export async function uploadLitter(litter: Litter): Promise<Id> {
    const options = {
        headers: JSON_HEADERS,
        method: "PUT",
        body: JSON.stringify({ litter }),
    };

    const res = await fetch(API_LITTER + "/new", options);
    const body: Id = await res.json();
    return body;
}

export async function updateLitter(litter: Litter): Promise<boolean> {
    const options = {
        headers: JSON_HEADERS,
        method: "PUT",
        body: JSON.stringify({ litter }),
    };

    const res = await fetch(API_LITTER + "/update", options);
    return res.ok;
}

export async function deleteLitter(id: number): Promise<boolean> {
    const options = {
        headers: JSON_HEADERS,
        method: "DELETE",
        body: JSON.stringify({ id }),
    };

    const res = await fetch(API_LITTER + "/delete", options);
    return res.ok;
}
