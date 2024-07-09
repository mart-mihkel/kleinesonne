import { fetchNames } from "$lib/mock-server";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = () => {
    return {
        names: fetchNames(),
    };
};
