import { fetchNews, resdata } from "$lib/api";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch }) => {
    const time = Math.floor(new Date().getTime() / 1000);
    const res = await fetchNews(time, 10, fetch);
    return resdata(res);
};
