import { fetchLitterNames, resdata } from "$lib/api";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async () => {
    const res = await fetchLitterNames();
    return resdata(res);
};
