import type { LayoutLoad } from "./$types";
import { browser } from "$app/environment";
import { waitLocale } from "svelte-i18n";
import { register, init } from "svelte-i18n";

export const load: LayoutLoad = async () => {
    register("ee", () => import("$lib/locales/ee.json"));
    register("en", () => import("$lib/locales/en.json"));

    const fallback = "ee";
    const storage = browser ? localStorage.getItem("locale") : fallback;
    const navigator = browser ? window.navigator.language : fallback;
    const initial = storage ?? navigator;

    init({
        fallbackLocale: fallback,
        initialLocale: initial,
    });

    await waitLocale();
};
