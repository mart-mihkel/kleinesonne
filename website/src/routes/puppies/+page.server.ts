import { fetchAvaialbleLitters } from "$lib/api";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async () => {
    return { litters: fetchAvaialbleLitters() };
};
