import type { Login } from "$lib/types";

export const API = "http://127.0.0.1:3000";

export const JSON_HEADERS = {
    "Content-Type": "application/json",
};

export async function authenticate(login: Login) {
    const options = {
        headers: JSON_HEADERS,
        method: "POST",
        body: JSON.stringify({ login }),
    };

    return fetch(API + "/auth", options);
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
    uploadPuppy,
    updatePuppy,
    deletePuppy,
} from "./puppy";

export {
    fetchLitterNames,
    fetchLitter,
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
