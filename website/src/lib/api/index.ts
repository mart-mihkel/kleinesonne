const API_AUTH = "http://127.0.0.1:3000/auth";

type Token = {
    token: string;
};

export async function authenticate(jwt: string): Promise<boolean> {
    const options = {
        headers: {
            "Content-Type": "application/json",
            Authorization: jwt,
        },
        method: "GET",
    };

    const res = await fetch(API_AUTH, options);
    return res.ok;
}

export async function login(user: string, secret: string): Promise<Token> {
    const options = {
        headers: { "Content-Type": "application/json" },
        method: "POST",
        body: JSON.stringify({ user, secret }),
    };

    const res = await fetch(API_AUTH, options);
    const body: Token = await res.json();
    return body;
}

export { API_UPLOADS, uploadImages, deleteImage } from "./uploads";

export {
    API_DOG,
    fetchDogNames,
    fetchDog,
    fetchAliveDogs,
    fetchRetiredDogs,
    uploadDog,
    updateDog,
    deleteDog,
} from "./dog";

export {
    API_PUPPY,
    fetchPuppyNames,
    fetchPuppy,
    fetchPuppies,
    fetchAvailablePuppies,
    uploadPuppy,
    updatePuppy,
    deletePuppy,
} from "./puppy";

export {
    API_LITTER,
    fetchLitterNames,
    fetchLitter,
    fetchAvaialbleLitters,
    fetchAvaialbleLittersByBreed,
    uploadLitter,
    updateLitter,
    deleteLitter,
} from "./litter";

export {
    API_NEWS,
    fetchTitles,
    fetchArticle,
    fetchNews,
    uploadArticle,
    updateArticle,
    deleteArticle,
} from "./news";
