import { resdata } from "$lib/api";
import { fetchNews } from "$lib/api/news";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch }) => {
    const time = new Date().getTime();
    const res = await fetchNews(time, 10, fetch);
    return resdata(res);
};
