import type { Actions, PageServerLoad } from "./$types";
import { fail } from "@sveltejs/kit";

export const load: PageServerLoad = ({ cookies }) => {
    const id = cookies.get("sessionid");
    const authenticated = id === "temp" ? true : false;

    return { authenticated };
};

export const actions: Actions = {
    login: async ({ request, cookies }) => {
        const data = await request.formData();
        const username = data.get("username");
        const password = data.get("password");

        if (username === null || password === null) {
            fail(401, { error: "Missing username or password" });
        }

        cookies.set("sessionid", "temp", { path: "/admin" });
    },
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
