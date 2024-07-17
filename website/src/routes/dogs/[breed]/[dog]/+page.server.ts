import { fetchDog, resdata } from "$lib/api";
import { fetchFamily } from "$lib/api/dog";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ params }) => {
    const id = params.dog;
    const res = await fetchDog(Number(id));
    const family = await fetchFamily(id);
    return { dog: resdata(res), family: resdata(family) };
};
