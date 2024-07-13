import { type Id, type Name, type Puppy } from "$lib/types";
import { API, JSON_HEADERS } from ".";

const API_PUPPY = `${API}/puppy`;

export async function fetchPuppyNames(litter_id: number): Promise<Name[]> {
    const options = {
        headers: JSON_HEADERS,
        method: "POST",
        body: JSON.stringify({ litter_id }),
    };

    const res = await fetch(API_PUPPY + "/names", options);
    const body: Name[] = await res.json();
    return body;
}

export async function fetchPuppies(litter_id: number): Promise<Puppy[]> {
    const options = {
        headers: JSON_HEADERS,
        method: "POST",
        body: JSON.stringify({ id: litter_id }),
    };

    const res = await fetch(API_PUPPY + "/litter", options);
    const body: Puppy[] = await res.json();
    return body;
}

export async function fetchAvailablePuppies(
    litter_id: number,
): Promise<Puppy[]> {
    const options = {
        headers: JSON_HEADERS,
        method: "POST",
        body: JSON.stringify({ id: litter_id }),
    };

    const res = await fetch(API_PUPPY + "/available", options);
    const body: Puppy[] = await res.json();
    return body;
}

export async function uploadPuppy(puppy: Puppy): Promise<Id> {
    const options = {
        headers: JSON_HEADERS,
        method: "PUT",
        body: JSON.stringify({ puppy }),
    };

    const res = await fetch(API_PUPPY + "/new", options);
    const body: Id = await res.json();
    return body;
}

export async function updatePuppy(puppy: Puppy): Promise<boolean> {
    const options = {
        headers: JSON_HEADERS,
        method: "PUT",
        body: JSON.stringify({ puppy }),
    };

    const res = await fetch(API_PUPPY + "/modify", options);
    return res.ok;
}

export async function deletePuppy(id: number): Promise<boolean> {
    const options = {
        headers: JSON_HEADERS,
        method: "DELETE",
        body: JSON.stringify({ id }),
    };

    const res = await fetch(API_PUPPY + "/delete", options);
    return res.ok;
}
