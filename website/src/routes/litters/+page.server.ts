import { resdata } from "$lib/api";
import { fetchLitterNames } from "$lib/api/litter";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch }) => {
    const res = await fetchLitterNames(fetch);
    return resdata(res);
};
