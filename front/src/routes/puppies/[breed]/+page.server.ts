import { fetchAvailableBreedPuppies } from "$lib/mock-server";
import { Breed } from "$lib/types";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = ({ params }) => {
    return { litters: fetchAvailableBreedPuppies(params.breed as Breed) };
};

