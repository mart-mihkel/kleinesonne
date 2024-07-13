import type { Login, Jwt } from "$lib/types";

export const API = "http://127.0.0.1:3000";
export const JSON_HEADERS = {
    "Content-Type": "application/json",
};

export async function authenticate(login: Login): Promise<Jwt> {
    const options = {
        headers: JSON_HEADERS,
        method: "POST",
        body: JSON.stringify({ login }),
    };

    const res = await fetch(API + "/auth", options);
    const body: Jwt = await res.json();
    return body;
}

export {
    fetchDogNames,
    fetchDog,
    fetchDogs,
    uploadDog,
    updateDog,
    deleteDog,
} from "./dog";

export {
    fetchPuppyNames,
    fetchPuppies,
    fetchAvailablePuppies,
    uploadPuppy,
    updatePuppy,
    deletePuppy,
} from "./puppy";

export {
    fetchLitterNames,
    fetchLitter,
    fetchAvaialbleLitters,
    uploadLitter,
    updateLitter,
    deleteLitter,
} from "./litter";

export {
    fetchTitles,
    fetchNews,
    uploadArticle,
    updateArticle,
    deleteArticle,
} from "./news";
