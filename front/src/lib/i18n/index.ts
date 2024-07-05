import { browser } from "$app/environment";
import { register, init } from "svelte-i18n";

register('en', () => import('./locales/en.json'))
register('ee', () => import('./locales/ee.json'))

init({
	fallbackLocale: "en",
	initialLocale: browser ? window.navigator.language : "en",
})
