import { fetchAvaialbleLittersByBreed, resdata } from "$lib/api";
import { Breed } from "$lib/types";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ params }) => {
    const breed = params.breed as Breed;
    const res = await fetchAvaialbleLittersByBreed(breed);
    return resdata(res);
};
