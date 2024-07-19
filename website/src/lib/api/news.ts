import type { Article, Name, Title, ApiResponse } from "$lib/types";

export const API_NEWS = "http://api:3000/news";

export async function fetchTitles(): Promise<ApiResponse<Name[]>> {
    const options = {
        headers: { "Content-Type": "application/json" },
        method: "GET",
    };

    const mapper = (t: Title) => ({ id: t.id, name: t.title });

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
): Promise<ApiResponse<Article[]>> {
    const options = {
        headers: { "Content-Type": "application/json" },
        method: "POST",
        body: JSON.stringify({ from, n }),
    };

    const res = await fetch(API_NEWS + "/from", options);
    const body = await res.json();
    return res.ok
        ? { res: { type: "data", data: body.data } }
        : { res: { type: "error", error: body.error } };
}

export async function uploadArticle(
    article: Article,
    jwt: string,
): Promise<ApiResponse<number>> {
    const options = {
        headers: { "Content-Type": "application/json", Authorization: jwt },
        method: "PUT",
        body: JSON.stringify(article),
    };

    const res = await fetch(API_NEWS + "/new", options);
    const body = await res.json();
    return res.ok
        ? { res: { type: "data", data: body.data } }
        : { res: { type: "error", error: body.error } };
}

export async function updateArticle(
    article: Article,
    jwt: string,
): Promise<ApiResponse<never>> {
    const options = {
        headers: { "Content-Type": "application/json", Authorization: jwt },
        method: "PUT",
        body: JSON.stringify(article),
    };

    const res = await fetch(API_NEWS + "/update", options);
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
