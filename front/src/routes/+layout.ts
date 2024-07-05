import type { LayoutLoad } from './$types';
import { browser } from '$app/environment';
import { waitLocale } from 'svelte-i18n';
import { register, init } from "svelte-i18n";

export const load: LayoutLoad = async () => {
    register('en', () => import('$lib/locales/en.json'))
    register('ee', () => import('$lib/locales/ee.json'))

    const fallback = "en";
    const storage = browser ? localStorage.getItem("locale") : fallback;
    const navigator = browser ? window.navigator.language : fallback;
    const initial = storage ?? navigator;

    init({
        fallbackLocale: fallback,
        initialLocale: initial,
    })

    await waitLocale();
}
