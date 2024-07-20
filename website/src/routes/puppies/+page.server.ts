import { fetchAvaialbleLitters, resdata } from "$lib/api";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch }) => {
    const res = await fetchAvaialbleLitters(fetch);
    return resdata(res);
};
