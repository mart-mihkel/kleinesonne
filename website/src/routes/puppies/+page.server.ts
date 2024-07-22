import { resdata } from "$lib/api";
import { fetchAvaialbleLitters } from "$lib/api/litter";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch }) => {
    const res = await fetchAvaialbleLitters(fetch);
    return resdata(res);
};
