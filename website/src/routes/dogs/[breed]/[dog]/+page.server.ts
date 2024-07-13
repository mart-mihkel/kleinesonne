import { fetchDog } from "$lib/api";
import { fetchFamily } from "$lib/api/dog";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ params }) => {
    const id = params.dog;
    return {
        dog: fetchDog(Number(id)),
        tree: fetchFamily(id),
    };
};
