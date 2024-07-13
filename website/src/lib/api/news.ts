import type { Article, Id, Title } from "$lib/types";
import { API, JSON_HEADERS } from ".";

const API_NEWS = `${API}/news`;

export async function fetchTitles(): Promise<Title[]> {
    const options = {
        headers: JSON_HEADERS,
        method: "GET",
    };

    const res = await fetch(API_NEWS + "/titles", options);
    const body: Title[] = await res.json();
    return body;
}

export async function fetchNews(from: Date, n: number): Promise<Article[]> {
    const options = {
        headers: JSON_HEADERS,
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

export async function uploadArticle(article: Article): Promise<Id> {
    const options = {
        headers: JSON_HEADERS,
        method: "PUT",
        body: JSON.stringify({ article }),
    };

    const res = await fetch(API_NEWS + "/new", options);
    const body: Id = await res.json();
    return body;
}

export async function updateArticle(article: Article): Promise<boolean> {
    const options = {
        headers: JSON_HEADERS,
        method: "PUT",
        body: JSON.stringify({ article }),
    };

    const res = await fetch(API_NEWS + "/update", options);
    return res.ok;
}

export async function deleteArticle(id: number): Promise<boolean> {
    const options = {
        headers: JSON_HEADERS,
        method: "DELETE",
        body: JSON.stringify({ id }),
    };

    const res = await fetch(API_NEWS + "/delete", options);
    return res.ok;
}
