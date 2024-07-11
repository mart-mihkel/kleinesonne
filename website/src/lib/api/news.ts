import type { Article } from "$lib/types";
import { API, JSON_HEADERS } from ".";

const API_NEWS = `${API}/news`;

export async function fetchTitles() {
    const options = {
        headers: JSON_HEADERS,
        method: "GET",
    };

    return fetch(API_NEWS + "/titles", options);
}

export async function fetchNews(from: Date, n: number): Promise<Response> {
    const body = JSON.stringify({
        from: Math.floor(from.getTime() / 1000),
        n,
    });

    const options = {
        headers: JSON_HEADERS,
        method: "POST",
        body,
    };

    return fetch(API_NEWS + "/from", options);
}

export async function uploadArticle(article: Article) {
    const options = {
        headers: JSON_HEADERS,
        method: "PUT",
        body: JSON.stringify({ article }),
    };

    return fetch(API_NEWS + "/new", options);
}

export async function updateArticle(article: Article) {
    const options = {
        headers: JSON_HEADERS,
        method: "PUT",
        body: JSON.stringify({ article }),
    };

    return fetch(API_NEWS + "/update", options);
}

export async function deleteArticle(id: number) {
    const options = {
        headers: JSON_HEADERS,
        method: "DELETE",
        body: JSON.stringify({ id }),
    };

    return fetch(API_NEWS + "/delete", options);
}
