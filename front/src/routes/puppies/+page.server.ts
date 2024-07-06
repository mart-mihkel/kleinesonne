import { fetchAvailablePuppies } from "$lib/mock-server";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = () => {
    return { litters: fetchAvailablePuppies() };
};

