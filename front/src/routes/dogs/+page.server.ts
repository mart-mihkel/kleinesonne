import { type Image } from "$lib/types";
import type { PageServerLoad } from "./$types";

export const prerender = true;

export const load: PageServerLoad = () => {
	const aus: Image[] = [
		{ src: "/rand.webp", alt: "rand" },
		{ src: "/rand.webp", alt: "rand" },
		{ src: "/rand.webp", alt: "rand" },
	];

	const entle: Image[] = [
		{ src: "/rand.webp", alt: "rand" },
		{ src: "/rand.webp", alt: "rand" },
		{ src: "/rand.webp", alt: "rand" },
	];

	const bern: Image[] = [
		{ src: "/rand.webp", alt: "rand" },
		{ src: "/rand.webp", alt: "rand" },
		{ src: "/rand.webp", alt: "rand" },
	];

	return { aus, entle, bern };
}
