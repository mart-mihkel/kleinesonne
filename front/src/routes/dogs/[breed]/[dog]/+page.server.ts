import { fetchDog, fetchFamily } from "$lib/mock-server";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ params }) => {
    const name = params.dog;
    return {
        dog: fetchDog(name),
        tree: fetchFamily(),
    };
};

