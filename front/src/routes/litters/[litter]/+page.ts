import { Availability, Breed, Gender, type Litter } from "$lib/types";
import type { PageLoad } from "./$types";

export const load: PageLoad = ({ params }) => {
	const litters: Litter[] = [
		{
			name: "Entlebuch Marakratid",
			parents: { src: "/parents.webp", alt: "parents" },
			breed: Breed.ENTLEBUCHER,
			puppies: [
				{
					image: { src: "/usin.webp", alt: "usin" },
					name: "Kleine Sonne Golf in Leuk",
					gender: Gender.MALE,
					availability: Availability.AVAILABLE,
				},
				{
					image: { src: "/usin.webp", alt: "usin" },
					name: "Kleine Sonne Golf in Davos",
					gender: Gender.MALE,
					availability: Availability.AVAILABLE,
				},
				{
					image: { src: "/usin.webp", alt: "usin" },
					name: "Kleine Sonne Golf in Erlen",
					gender: Gender.FEMALE,
					availability: Availability.CO_OWNERSHIP,
				},
				{
					image: { src: "/usin.webp", alt: "usin" },
					name: "Kleine Sonne Golf in Freakazoid",
					gender: Gender.FEMALE,
					availability: Availability.UNAVAILABLE,
				},
			],
		},
		{
			name: "Australian Mürakarud",
			parents: { src: "/parents_aasa.webp", alt: "parents aasa" },
			breed: Breed.AUSTRALIAN,
			puppies: [
				{
					image: { src: "/rand.webp", alt: "rand" },
					name: "Kleine Sonne Ice Golem",
					gender: Gender.MALE,
					availability: Availability.AVAILABLE,
				},
				{
					image: { src: "/rand.webp", alt: "rand" },
					name: "Kleine Sonne Green Goblin",
					gender: Gender.FEMALE,
					availability: Availability.CO_OWNERSHIP,
				},
				{
					image: { src: "/rand.webp", alt: "rand" },
					name: "Kleine Sonne All Might",
					gender: Gender.FEMALE,
					availability: Availability.UNAVAILABLE,
				},
			],
		},
	]

	const litter = litters.find(l => l.name === params.litter);
	const names = litters.map(l => l.name);

	return { names, litter };
}
