import type { NewsPiece } from "$lib/types";
import type { PageLoad } from "./$types";

export const load: PageLoad = () => {
	const news: NewsPiece[] = [
		{
			title: "Karm koer",
			date: new Date(2018, 5, 12),
			images: [],
			text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Morbi porttitor sapien non bibendum tincidunt. Vivamus tincidunt lorem dui, ac fringilla dolor porta quis. Vivamus nec tortor ac lorem molestie congue. Mauris ullamcorper id sapien id mattis. In at iaculis dolor.",
		},
		{
			title: "Rabaretk",
			date: new Date(2018, 4, 12),
			images: [
				{ src: "/rand.webp", alt: "rand" },
			],
			text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Morbi porttitor sapien non bibendum tincidunt. Vivamus tincidunt lorem dui, ac fringilla dolor porta quis. Vivamus nec tortor ac lorem molestie congue. Mauris ullamcorper id sapien id mattis. In at iaculis dolor.",

		},
		{
			title: "Kohalik karujüri",
			date: new Date(2018, 4, 3),
			images: [
				{ src: "/usin.webp", alt: "usin" },
				{ src: "/viisakas.webp", alt: "usin" },
				{ src: "/rand.webp", alt: "usin" },
			],
			text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Morbi porttitor sapien non bibendum tincidunt. Vivamus tincidunt lorem dui, ac fringilla dolor porta quis. Vivamus nec tortor ac lorem molestie congue. Mauris ullamcorper id sapien id mattis. In at iaculis dolor.",
		}
	];

	return { news }
};
