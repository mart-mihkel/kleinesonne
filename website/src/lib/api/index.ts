import type { ApiResponse, ResponseData } from "$lib/types";

export const UNEXPECTED_ERROR = { error: "Unexpected server response" };

export function resdata<D>({ res }: ApiResponse<D>): ResponseData<D> {
    if (res.type === "error") {
        return { error: res.error };
    }

    if (res.type !== "data") {
        return UNEXPECTED_ERROR;
    }

    return { data: res.data };
}

export { authenticate, login } from "./auth";

export { API_UPLOADS, uploadImages, deleteImage } from "./uploads";

export {
    API_DOG,
    fetchDogNames,
    fetchDog,
    fetchAliveDogs,
    fetchRetiredDogs,
    fetchFamily,
    updateFamily,
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
