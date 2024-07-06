import { fetchNews } from "$lib/mock-server";
import type { PageServerLoad } from "./$types";


export const load: PageServerLoad = async () => {
    return { news: fetchNews(new Date()) };
};

