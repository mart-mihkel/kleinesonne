import { type Image } from "$lib/types";
import type { PageLoad } from "./$types";

export const load: PageLoad = () => {
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
