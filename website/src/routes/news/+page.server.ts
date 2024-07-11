import { fetchNews } from "$lib/api";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async () => {
    return { news: fetchNews(new Date(), 5) };
};
