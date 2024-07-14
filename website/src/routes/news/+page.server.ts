import { fetchNews } from "$lib/api";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async () => {
    const time = Math.floor(new Date().getTime() / 1000);
    return { news: fetchNews(time, 10) };
};
