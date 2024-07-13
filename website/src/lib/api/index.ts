import type { Login, Jwt } from "$lib/types";

const API_AUTH = "http://127.0.0.1:3000/auth";

export async function authenticate(login: Login): Promise<Jwt> {
    const options = {
        headers: { "Content-Type": "application/json" },
        method: "POST",
        body: JSON.stringify({ login }),
    };

    const res = await fetch(API_AUTH, options);
    const body: Jwt = await res.json();
    return body;
}

export {
    fetchDogNames,
    fetchDog,
    fetchAliveDogs,
    fetchRetiredDogs,
    uploadDog,
    updateDog,
    deleteDog,
} from "./dog";

export {
    fetchPuppyNames,
    fetchPuppy,
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
    fetchAvaialbleLittersByBreed,
    uploadLitter,
    updateLitter,
    deleteLitter,
} from "./litter";

export {
    fetchTitles,
    fetchArticle,
    fetchNews,
    uploadArticle,
    updateArticle,
    deleteArticle,
} from "./news";
