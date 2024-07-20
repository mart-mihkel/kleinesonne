import { fetchDog, resdata } from "$lib/api";
import { fetchFamily } from "$lib/api/dog";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ params, fetch }) => {
    const id = params.dog;
    const res = await fetchDog(Number(id), fetch);
    const family = await fetchFamily(fetch, id);
    return { dog: resdata(res), family: resdata(family) };
};
