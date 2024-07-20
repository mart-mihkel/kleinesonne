import { fetchLitterNames, resdata } from "$lib/api";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch }) => {
    const res = await fetchLitterNames(fetch);
    return resdata(res);
};
