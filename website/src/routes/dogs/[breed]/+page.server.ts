import { fetchAliveDogs, fetchRetiredDogs } from "$lib/api";
import { Breed, Gender } from "$lib/types";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ params }) => {
    const breed = params.breed as Breed;
    return {
        male: fetchAliveDogs(breed, Gender.MALE),
        female: fetchAliveDogs(breed, Gender.FEMALE),
        retired: fetchRetiredDogs(breed),
    };
};
