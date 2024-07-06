import type { Actions } from "./$types";

export const actions: Actions = {
    newsCreate: async ({ request }) => {
        const data = await request.formData();
        console.log(data);
    },
    newsUpdate: async ({ request }) => {
        const data = await request.formData();
        console.log(data);
    },
}
