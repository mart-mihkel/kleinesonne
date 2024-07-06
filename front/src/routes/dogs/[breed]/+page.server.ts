import { fetchBreedDogs, fetchBreedDogsRetired } from "$lib/mock-server";
import { Breed, Gender } from "$lib/types";
import type { PageServerLoad } from "./$types";


export const load: PageServerLoad = async ({ params }) => {
    const breed = params.breed as Breed;
    return {
        male: fetchBreedDogs(breed, Gender.MALE),
        female: fetchBreedDogs(breed, Gender.FEMALE),
        retired: fetchBreedDogsRetired(breed),
    };
};

