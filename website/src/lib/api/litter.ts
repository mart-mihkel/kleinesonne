import { type Litter } from "$lib/types";
import { API, JSON_HEADERS } from ".";

const API_LITTER = `${API}/litter`;

export async function fetchLitterNames(): Promise<Response> {
    const options = {
        headers: JSON_HEADERS,
        method: "GET",
    };

    return fetch(API_LITTER + "/names", options);
}

export async function fetchLitter(id: number): Promise<Response> {
    const options = {
        headers: JSON_HEADERS,
        method: "POST",
        body: JSON.stringify({ id }),
    };

    return fetch(API_LITTER + "/one", options);
}

export async function uploadLitter(litter: Litter) {
    const options = {
        headers: JSON_HEADERS,
        method: "PUT",
        body: JSON.stringify({ litter }),
    };

    return fetch(API_LITTER + "/new", options);
}

export async function updateLitter(litter: Litter) {
    const options = {
        headers: JSON_HEADERS,
        method: "PUT",
        body: JSON.stringify({ litter }),
    };

    return fetch(API_LITTER + "/update", options);
}

export async function deleteLitter(id: number) {
    const options = {
        headers: JSON_HEADERS,
        method: "DELETE",
        body: JSON.stringify({ id }),
    };

    return fetch(API_LITTER + "/delete", options);
}
