import { type Id, type Name, type Puppy } from "$lib/types";

const API_PUPPY = "http://127.0.0.1:3000/puppy";

export async function fetchPuppyNames(litter_id: number): Promise<Name[]> {
    const options = {
        headers: { "Content-Type": "application/json" },
        method: "POST",
        body: JSON.stringify({ litter_id }),
    };

    const res = await fetch(API_PUPPY + "/names", options);
    const body: Name[] = await res.json();
    return body;
}

export async function fetchPuppy(id: number): Promise<Puppy> {
    const options = {
        headers: { "Content-Type": "application/json" },
        method: "POST",
        body: JSON.stringify({ id }),
    };

    const res = await fetch(API_PUPPY + "/one", options);
    const body: Puppy = await res.json();
    return body;
}

export async function fetchPuppies(litter_id: number): Promise<Puppy[]> {
    const options = {
        headers: { "Content-Type": "application/json" },
        method: "POST",
        body: JSON.stringify({ litter_id }),
    };

    const res = await fetch(API_PUPPY + "/litter", options);
    const body: Puppy[] = await res.json();
    return body;
}

export async function fetchAvailablePuppies(
    litter_id: number,
): Promise<Puppy[]> {
    const options = {
        headers: { "Content-Type": "application/json" },
        method: "POST",
        body: JSON.stringify({ litter_id }),
    };

    const res = await fetch(API_PUPPY + "/available", options);
    const body: Puppy[] = await res.json();
    return body;
}

export async function uploadPuppy(puppy: Puppy): Promise<Id> {
    const options = {
        headers: { "Content-Type": "application/json" },
        method: "PUT",
        body: JSON.stringify({ puppy }),
    };

    const res = await fetch(API_PUPPY + "/new", options);
    const body: Id = await res.json();
    return body;
}

export async function updatePuppy(puppy: Puppy): Promise<boolean> {
    const options = {
        headers: { "Content-Type": "application/json" },
        method: "PUT",
        body: JSON.stringify({ puppy }),
    };

    const res = await fetch(API_PUPPY + "/modify", options);
    return res.ok;
}

export async function deletePuppy(id: number): Promise<boolean> {
    const options = {
        headers: { "Content-Type": "application/json" },
        method: "DELETE",
        body: JSON.stringify({ id }),
    };

    const res = await fetch(API_PUPPY + "/delete", options);
    return res.ok;
}
