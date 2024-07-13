import { fetchDog } from "$lib/api";
import { fetchFamily } from "$lib/api/dog";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ params }) => {
    const id = 0; // TODO: get actual id from load params
    const name = params.dog;
    return {
        dog: fetchDog(id),
        tree: fetchFamily(name),
    };
};
