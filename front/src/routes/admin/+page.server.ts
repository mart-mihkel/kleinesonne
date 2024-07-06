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
    dogCreate: async ({ request }) => {
        const data = await request.formData();
        console.log(data);
    },
    dogUpdate: async ({ request }) => {
        const data = await request.formData();
        console.log(data);
    },
    litterCreate: async ({ request }) => {
        const data = await request.formData();
        console.log(data);
    },
    litterUpdate: async ({ request }) => {
        const data = await request.formData();
        console.log(data);
    },
    puppyCreate: async ({ request }) => {
        const data = await request.formData();
        console.log(data);
    },
    puppyUpdate: async ({ request }) => {
        const data = await request.formData();
        console.log(data);
    },
};
