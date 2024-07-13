import { fetchAvaialbleLitters } from "$lib/api";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async () => {
    const litters = await fetchAvaialbleLitters();

    return { litters };
};
