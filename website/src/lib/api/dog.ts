import type { Breed, Dog, Id, Name } from "$lib/types";
import { API, JSON_HEADERS } from ".";

const API_DOG = `${API}/dog`;

export async function fetchDogNames(): Promise<Name[]> {
    const options = {
        headers: JSON_HEADERS,
        method: "GET",
    };

    const res = await fetch(API_DOG + "/names", options);
    const body: Name[] = await res.json();
    return body;
}

export async function fetchDog(id: number): Promise<Dog> {
    const options = {
        headers: JSON_HEADERS,
        method: "POST",
        body: JSON.stringify({ id }),
    };

    const res = await fetch(API_DOG + "/one", options);
    const body: Dog = await res.json();
    return body;
}

export async function fetchDogs(breed: Breed, alive: boolean): Promise<Dog[]> {
    const options = {
        headers: JSON_HEADERS,
        method: "POST",
        body: JSON.stringify({
            breed,
            alive,
        }),
    };

    const res = await fetch(API_DOG + "/breed", options);
    const body: Dog[] = await res.json();
    return body;
}

export async function uploadDog(dog: Dog): Promise<Id> {
    const options = {
        headers: JSON_HEADERS,
        method: "PUT",
        body: JSON.stringify({ dog }),
    };

    const res = await fetch(API_DOG + "/new", options);
    const body: Id = await res.json();
    return body;
}

export async function updateDog(dog: Dog): Promise<boolean> {
    const options = {
        headers: JSON_HEADERS,
        method: "PUT",
        body: JSON.stringify({ dog }),
    };

    const res = await fetch(API_DOG + "/update", options);
    return res.ok;
}

export async function deleteDog(id: number): Promise<boolean> {
    const options = {
        headers: JSON_HEADERS,
        method: "DELETE",
        body: JSON.stringify({ id }),
    };

    const res = await fetch(API_DOG + "/delete", options);
    return res.ok;
}
