import { fetchAvaialbleLittersByBreed } from "$lib/api";
import { Breed } from "$lib/types";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ params }) => {
    const breed = params.breed as Breed;
    return { litters: fetchAvaialbleLittersByBreed(breed) };
};
