import type { ApiResponse, Article, SsrFetch } from "$lib/types";
import { SSR_NEWS } from "$lib/api/news";

export async function uploadArticle(
    fetch: SsrFetch,
    article: Article,
    jwt: string,
): Promise<ApiResponse<number>> {
    const options = {
        headers: { "Content-Type": "application/json", Authorization: jwt },
        method: "PUT",
        body: JSON.stringify(article),
    };

    const res = await fetch(SSR_NEWS + "/new", options);
    const body = await res.json();
    return res.ok
        ? { res: { type: "data", data: body.data } }
        : { res: { type: "error", error: body.error } };
}

export async function updateArticle(
    fetch: SsrFetch,
    article: Article,
    jwt: string,
): Promise<ApiResponse<never>> {
    const options = {
        headers: { "Content-Type": "application/json", Authorization: jwt },
        method: "PUT",
        body: JSON.stringify(article),
    };

    const res = await fetch(SSR_NEWS + "/update", options);
    const body = await res.json();
    return res.ok
        ? { res: { type: "success" } }
        : { res: { type: "error", error: body.error } };
}
