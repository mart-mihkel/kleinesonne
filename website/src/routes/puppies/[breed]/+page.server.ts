import { fetchAvaialbleLittersByBreed, fetchAvailablePuppies } from "$lib/api";
import { Breed, type LitterWithPuppies } from "$lib/types";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ params }) => {
    const breed = params.breed as Breed;
    const litters = await fetchAvaialbleLittersByBreed(breed);

    return {
        litters: litters.map(async (l): Promise<LitterWithPuppies> => {
            const puppies = await fetchAvailablePuppies(l.id);
            return { ...l, puppies };
        }),
    };
};
