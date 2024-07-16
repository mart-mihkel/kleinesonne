import type { Breed, Dog, Family, Gender, Id, Name } from "$lib/types";

export const API_DOG = "http://127.0.0.1:3000/dog";

export async function fetchFamily(name: string): Promise<Family> {
    // TODO: graphdb
    return { name };
}

export async function fetchDogNames(): Promise<Name[]> {
    const options = {
        headers: { "Content-Type": "application/json" },
        method: "GET",
    };

    const res = await fetch(API_DOG + "/names", options);
    const body: Name[] = await res.json();
    return body;
}

export async function fetchDog(id: number): Promise<Dog> {
    const options = {
        headers: { "Content-Type": "application/json" },
        method: "POST",
        body: JSON.stringify({ id }),
    };

    const res = await fetch(API_DOG + "/one", options);
    const body: Dog = await res.json();
    return body;
}

export async function fetchAliveDogs(
    breed: Breed,
    gender: Gender,
): Promise<Dog[]> {
    const options = {
        headers: { "Content-Type": "application/json" },
        method: "POST",
        body: JSON.stringify({
            breed,
            gender,
        }),
    };

    const res = await fetch(API_DOG + "/alive", options);
    const body: Dog[] = await res.json();
    return body;
}

export async function fetchRetiredDogs(breed: Breed): Promise<Dog[]> {
    const options = {
        headers: { "Content-Type": "application/json" },
        method: "POST",
        body: JSON.stringify({
            breed,
        }),
    };

    const res = await fetch(API_DOG + "/retired", options);
    const body: Dog[] = await res.json();
    return body;
}

export async function uploadDog(dog: Dog, jwt: string): Promise<Id> {
    const options = {
        headers: { "Content-Type": "application/json", Authorization: jwt },
        method: "PUT",
        body: JSON.stringify(dog),
    };

    const res = await fetch(API_DOG + "/new", options);
    const body: Id = await res.json();
    return body;
}

export async function updateDog(dog: Dog, jwt: string): Promise<boolean> {
    const options = {
        headers: { "Content-Type": "application/json", Authorization: jwt },
        method: "PUT",
        body: JSON.stringify(dog),
    };

    const res = await fetch(API_DOG + "/update", options);
    return res.ok;
}

export async function deleteDog(id: number, jwt: string): Promise<boolean> {
    const options = {
        headers: { "Content-Type": "application/json", Authorization: jwt },
        method: "DELETE",
        body: JSON.stringify({ id }),
    };

    const res = await fetch(API_DOG + "/delete", options);
    return res.ok;
}
