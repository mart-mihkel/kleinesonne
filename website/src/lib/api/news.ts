import type { Article, Id, Name, Title } from "$lib/types";

const API_NEWS = "http://127.0.0.1:3000/news";

export async function fetchTitles(): Promise<Name[]> {
    const options = {
        headers: { "Content-Type": "application/json" },
        method: "GET",
    };

    const res = await fetch(API_NEWS + "/titles", options);
    const body: Title[] = await res.json();
    return body.map((t) => ({ id: t.id, name: t.title }));
}

export async function fetchArticle(id: number): Promise<Article> {
    const options = {
        headers: { "Content-Type": "application/json" },
        method: "POST",
        body: JSON.stringify({ id }),
    };

    const res = await fetch(API_NEWS + "/one", options);
    const body: Article = await res.json();
    return body;
}

export async function fetchNews(from: Date, n: number): Promise<Article[]> {
    const options = {
        headers: { "Content-Type": "application/json" },
        method: "POST",
        body: JSON.stringify({
            from: Math.floor(from.getTime() / 1000),
            n,
        }),
    };

    const res = await fetch(API_NEWS + "/from", options);
    const body: Article[] = await res.json();
    return body;
}

export async function uploadArticle(
    article: Article,
    jwt: string,
): Promise<Id> {
    const options = {
        headers: { "Content-Type": "application/json", Authorization: jwt },
        method: "PUT",
        body: JSON.stringify(article),
    };

    const res = await fetch(API_NEWS + "/new", options);
    const body: Id = await res.json();
    return body;
}

export async function updateArticle(
    article: Article,
    jwt: string,
): Promise<boolean> {
    const options = {
        headers: { "Content-Type": "application/json", Authorization: jwt },
        method: "PUT",
        body: JSON.stringify(article),
    };

    const res = await fetch(API_NEWS + "/update", options);
    return res.ok;
}

export async function deleteArticle(id: number, jwt: string): Promise<boolean> {
    const options = {
        headers: { "Content-Type": "application/json", Authorization: jwt },
        method: "DELETE",
        body: JSON.stringify({ id }),
    };

    const res = await fetch(API_NEWS + "/delete", options);
    return res.ok;
}
