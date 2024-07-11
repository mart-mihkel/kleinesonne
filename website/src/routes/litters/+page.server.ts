import { fetchLitterNames } from "$lib/api";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = () => {
    return {
        names: fetchLitterNames(),
    };
};
