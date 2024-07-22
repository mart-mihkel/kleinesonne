import { resdata } from "$lib/api";
import { fetchAliveDogs, fetchRetiredDogs } from "$lib/api/dog";
import { Breed, Gender } from "$lib/enums";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ params, fetch }) => {
    const breed = params.breed as Breed;

    const male = await fetchAliveDogs(breed, Gender.MALE, fetch);
    const female = await fetchAliveDogs(breed, Gender.FEMALE, fetch);
    const retired = await fetchRetiredDogs(breed, fetch);

    return { data: [resdata(male), resdata(female), resdata(retired)] };
};
