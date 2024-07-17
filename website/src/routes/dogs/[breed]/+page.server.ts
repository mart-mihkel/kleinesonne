import { fetchAliveDogs, fetchRetiredDogs, resdata } from "$lib/api";
import { Breed, Gender } from "$lib/types";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ params }) => {
    const breed = params.breed as Breed;

    const male = await fetchAliveDogs(breed, Gender.MALE);
    const female = await fetchAliveDogs(breed, Gender.FEMALE);
    const retired = await fetchRetiredDogs(breed);

    return { data: [resdata(male), resdata(female), resdata(retired)] };
};
