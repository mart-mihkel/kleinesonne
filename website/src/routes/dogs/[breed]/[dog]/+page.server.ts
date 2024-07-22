import { resdata } from "$lib/api";
import { fetchDog } from "$lib/api/dog";
import { fetchFamily } from "$lib/server/dog";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ params, fetch }) => {
    const id = params.dog;
    const dogr = await fetchDog(Number(id), fetch);
    const dog = resdata(dogr);

    if (dog.error || !dog.data) {
        return {
            dog,
            family: { data: undefined, error: "Failed to fetch dog" },
        };
    }

    const familyr = await fetchFamily(dog.data.name);
    const family = resdata(familyr);

    return { dog, family };
};
