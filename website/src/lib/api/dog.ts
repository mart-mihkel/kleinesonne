import type { Breed, Dog } from "$lib/types";
import { API, JSON_HEADERS } from ".";

const API_DOG = `${API}/dog`;

export async function fetchDogNames(): Promise<Response> {
    const options = {
        headers: JSON_HEADERS,
        method: "GET",
    };

    return fetch(API_DOG + "/names", options);
}

export async function fetchDog(id: number): Promise<Response> {
    const options = {
        headers: JSON_HEADERS,
        method: "POST",
        body: JSON.stringify({ id }),
    };

    return fetch(API_DOG + "/one", options);
}

export async function fetchDogs(
    breed: Breed,
    alive: boolean,
): Promise<Response> {
    const body = JSON.stringify({
        breed,
        alive,
    });

    const options = {
        headers: JSON_HEADERS,
        method: "POST",
        body,
    };

    return fetch(API_DOG + "/breed", options);
}

export async function uploadDog(dog: Dog) {
    const options = {
        headers: JSON_HEADERS,
        method: "PUT",
        body: JSON.stringify({ dog }),
    };

    return fetch(API_DOG + "/new", options);
}

export async function updateDog(dog: Dog) {
    const options = {
        headers: JSON_HEADERS,
        method: "PUT",
        body: JSON.stringify({ dog }),
    };

    return fetch(API_DOG + "/update", options);
}

export async function deleteDog(id: number) {
    const options = {
        headers: JSON_HEADERS,
        method: "DELETE",
        body: JSON.stringify({ id }),
    };

    return fetch(API_DOG + "/delete", options);
}
