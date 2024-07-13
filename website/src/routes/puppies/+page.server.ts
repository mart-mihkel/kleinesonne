import { fetchAvaialbleLitters, fetchAvailablePuppies } from "$lib/api";
import type { LitterWithPuppies } from "$lib/types";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async () => {
    const litters = await fetchAvaialbleLitters();

    return {
        litters: litters.map(async (l): Promise<LitterWithPuppies> => {
            const puppies = await fetchAvailablePuppies(l.id);
            return { ...l, puppies };
        }),
    };
};
