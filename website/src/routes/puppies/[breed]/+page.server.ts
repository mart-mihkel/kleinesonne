import { fetchAvaialbleLittersByBreed, resdata } from "$lib/api";
import { Breed } from "$lib/enums";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ params, fetch }) => {
    const breed = params.breed as Breed;
    const res = await fetchAvaialbleLittersByBreed(breed, fetch);
    return resdata(res);
};
