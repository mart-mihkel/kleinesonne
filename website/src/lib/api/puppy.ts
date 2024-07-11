import { type Puppy } from "$lib/types";
import { API, JSON_HEADERS } from ".";

const API_PUPPY = `${API}/puppy`;

export async function fetchPuppyNames(litter_id: number): Promise<Response> {
    const options = {
        headers: JSON_HEADERS,
        method: "POST",
        body: JSON.stringify({ litter_id }),
    };

    return fetch(API_PUPPY + "/names", options);
}

export async function fetchPuppies(litter_id: number): Promise<Response> {
    const options = {
        headers: JSON_HEADERS,
        method: "POST",
        body: JSON.stringify({ id: litter_id }),
    };

    return fetch(API_PUPPY + "/litter", options);
}

export async function uploadPuppy(puppy: Puppy) {
    const options = {
        headers: JSON_HEADERS,
        method: "PUT",
        body: JSON.stringify({ puppy }),
    };

    return fetch(API_PUPPY + "/new", options);
}

export async function updatePuppy(puppy: Puppy) {
    const options = {
        headers: JSON_HEADERS,
        method: "PUT",
        body: JSON.stringify({ puppy }),
    };

    return fetch(API_PUPPY + "/modify", options);
}

export async function deletePuppy(id: number) {
    const options = {
        headers: JSON_HEADERS,
        method: "DELETE",
        body: JSON.stringify({ id }),
    };

    return fetch(API_PUPPY + "/delete", options);
}
