import type { Article, Name, ApiResponse, SsrFetch } from "$lib/types";

export const API_NEWS = "/api/news";
const SSR_NEWS = "http://api:3000/news";

export async function fetchTitles(): Promise<ApiResponse<Name[]>> {
    const options = {
        headers: { "Content-Type": "application/json" },
        method: "GET",
    };

    type Title = { id: number; title: string };
    const mapper = (t: Title): Name => ({ id: t.id, name: t.title });

    const res = await fetch(API_NEWS + "/titles", options);
    const body = await res.json();
    return res.ok
        ? { res: { type: "data", data: body.data.map(mapper) } }
        : { res: { type: "error", error: body.error } };
}

export async function fetchArticle(id: number): Promise<ApiResponse<Article>> {
    const options = {
        headers: { "Content-Type": "application/json" },
        method: "POST",
        body: JSON.stringify({ id }),
    };

    const res = await fetch(API_NEWS + "/one", options);
    const body = await res.json();
    return res.ok
        ? { res: { type: "data", data: body.data } }
        : { res: { type: "error", error: body.error } };
}

export async function fetchNews(
    from: number,
    n: number,
    ssr?: SsrFetch,
): Promise<ApiResponse<Article[]>> {
    const options = {
        headers: { "Content-Type": "application/json" },
        method: "POST",
        body: JSON.stringify({ from, n }),
    };

    const res = ssr
        ? await ssr(SSR_NEWS + "/from", options)
        : await fetch(API_NEWS + "/from", options);

    const body = await res.json();
    return res.ok
        ? { res: { type: "data", data: body.data } }
        : { res: { type: "error", error: body.error } };
}

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

export async function deleteArticle(
    id: number,
    jwt: string,
): Promise<ApiResponse<never>> {
    const options = {
        headers: { "Content-Type": "application/json", Authorization: jwt },
        method: "DELETE",
        body: JSON.stringify({ id }),
    };

    const res = await fetch(API_NEWS + "/delete", options);
    const body = await res.json();
    return res.ok
        ? { res: { type: "success" } }
        : { res: { type: "error", error: body.error } };
}
