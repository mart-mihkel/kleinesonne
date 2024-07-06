import { Availability, Breed, Gender, type Dog, type DogPreview, type Family, type Litter, type NewsPiece } from "$lib/types";

const PREVIEWS: DogPreview[] = [
	{
		thumbnail: { src: "/test.jpg", alt: "rand" },
		nickname: "Salsa",
		name: "Seventy Seven Spicy Salsa",
		breed: Breed.AUSTRALIAN,
		gender: Gender.FEMALE,
		alive: true,
	},
	{
		thumbnail: { src: "/test.jpg", alt: "kohver" },
		nickname: "Katja",
		name: "Korolevstvo Gornih Psov Okatava",
		breed: Breed.ENTLEBUCH,
		gender: Gender.FEMALE,
		alive: true,
	},
];

const SALSA: Dog = {
	thumbnail: { src: "/test.jpg", alt: "rand" },
	images: [
		{ src: "/test.jpg", alt: "rand" },
		{ src: "/test.jpg", alt: "rand" },
		{ src: "/test.jpg", alt: "rand" },
		{ src: "/test.jpg", alt: "rand" },
		{ src: "/test.jpg", alt: "rand" },
	],
	name: "Seventy Seven Spicy Salsa",
	nickname: "Salsa",
	father: "Peak River Fly Glossy Swiftlet",
	mother: "Seventy Seven Silver Bullet",
	dob: new Date(2016, 4, 22),
	breed: Breed.AUSTRALIAN,
	gender: Gender.FEMALE,
	alive: true,
	awards: ["World Winner show 2017 Class Winner", "3 CACIB", "2 res. CACIB"],
	health: [
		"HD-AA",
		"ED-00",
		"MDR1 +/+",
		"CEA carrier",
		"HC clear by parentage",
		"PRA clear by parentage",
		"Eyes clear",
	],
};

const KATJA: Dog = {
	thumbnail: { src: "/test.jpg", alt: "rand" },
	images: [
		{ src: "/test.jpg", alt: "rand" },
		{ src: "/test.jpg", alt: "rand" },
		{ src: "/test.jpg", alt: "rand" },
		{ src: "/test.jpg", alt: "rand" },
		{ src: "/test.jpg", alt: "rand" },
	],
	name: "Korolevstvo Gornih Psov Okatava",
	nickname: "Katja",
	father: "Peak River Fly Glossy Swiftlet",
	mother: "Seventy Seven Silver Bullet",
	dob: new Date(2014, 5, 22),
	breed: Breed.ENTLEBUCH,
	gender: Gender.FEMALE,
	alive: true,
	awards: ["World Winner show 2018 Class Winner", "CACIB", "Jäätise Meister"],
	health: [
		"HD-AA",
		"ED-00",
		"MDR1 +/+",
		"CEA carrier",
		"Built different",
		"PRA clear by parentage",
		"Eyes soggy",
	],
};

const FAMILY: Family = {
	name: "Kleine Sonne Awesome Huntress",
	father: {
		name: "Snowbelts Winning Ticket",
		father: {
			name: "Dreamstreets Season Ticket",
			father: { name: "Briarbrooks Valedictorian" },
			mother: { name: "Mysharas Shameless" },
		},
		mother: {
			name: "Mysharas Oh What a Night",
			father: { name: "Briarbrooks Copyright" },
			mother: { name: "Briarbrooks Finerthingsinlife" },
		},
	},
	mother: {
		name: "Bleuroyal Creamy Vodkanilla",
		father: {
			name: "Thornapple Uncle Sam Wants You",
			father: { name: "Thornapple Single Barrel" },
			mother: { name: "Thornapple Russian To a Party" },
		},
		mother: {
			name: "Thornapple Peachy Keen",
			father: { name: "Thornapple Hot Temptation" },
			mother: { name: "Thornapple Pieces Of My Hearth" },
		},
	},
};

const LITTERS: Litter[] = [
	{
		name: "Entlebuch Marakratid",
		parents: { src: "/test.jpg", alt: "parents" },
		breed: Breed.ENTLEBUCH,
		images: [],
		puppies: [
			{
				image: { src: "/test.jpg", alt: "usin" },
				name: "Kleine Sonne Golf in Leuk",
				gender: Gender.MALE,
				availability: Availability.AVAILABLE,
			},
			{
				image: { src: "/test.jpg", alt: "usin" },
				name: "Kleine Sonne Golf in Davos",
				gender: Gender.MALE,
				availability: Availability.AVAILABLE,
			},
			{
				image: { src: "/test.jpg", alt: "usin" },
				name: "Kleine Sonne Golf in Erlen",
				gender: Gender.FEMALE,
				availability: Availability.CO_OWNERSHIP,
			},
			{
				image: { src: "/test.jpg", alt: "usin" },
				name: "Kleine Sonne Golf in Freakazoid",
				gender: Gender.FEMALE,
				availability: Availability.UNAVAILABLE,
			},
		],
	},
	{
		name: "Australian Mürakarud",
		parents: { src: "/test.jpg", alt: "parents aasa" },
		breed: Breed.AUSTRALIAN,
		images: [
			{ src: "/test.jpg", alt: "usin" },
			{ src: "/test.jpg", alt: "usin" },
			{ src: "/test.jpg", alt: "usin" },
			{ src: "/test.jpg", alt: "usin" },
			{ src: "/test.jpg", alt: "usin" },
		],
		puppies: [
			{
				image: { src: "/test.jpg", alt: "usin" },
				name: "Kleine Sonne Ice Golem",
				gender: Gender.MALE,
				availability: Availability.AVAILABLE,
			},
			{
				image: { src: "/test.jpg", alt: "usin" },
				name: "Kleine Sonne Green Goblin",
				gender: Gender.FEMALE,
				availability: Availability.CO_OWNERSHIP,
			},
			{
				image: { src: "/test.jpg", alt: "usin" },
				name: "Kleine Sonne All Might",
				gender: Gender.FEMALE,
				availability: Availability.UNAVAILABLE,
			},
		],
	},
	{
		name: "Olematud Olendid",
		parents: { src: "", alt: "pole olemas" },
		breed: Breed.AUSTRALIAN,
		images: [],
		puppies: [
			{
				image: { src: "/test.jpg", alt: "usin" },
				name: "Kleine Sonne Not Real",
				gender: Gender.MALE,
				availability: Availability.CO_OWNERSHIP,
			},
		],
	},
];

const NEWS: NewsPiece[] = [
	{
		title: "Karm koer",
		date: new Date(2018, 5, 12),
		images: [],
		text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Morbi porttitor sapien non bibendum tincidunt. Vivamus tincidunt lorem dui, ac fringilla dolor porta quis. Vivamus nec tortor ac lorem molestie congue. Mauris ullamcorper id sapien id mattis. In at iaculis dolor.",
	},
	{
		title: "Rabaretk",
		date: new Date(2018, 4, 12),
		images: [{ src: "/test.jpg", alt: "rand" }],
		text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Morbi porttitor sapien non bibendum tincidunt. Vivamus tincidunt lorem dui, ac fringilla dolor porta quis. Vivamus nec tortor ac lorem molestie congue. Mauris ullamcorper id sapien id mattis. In at iaculis dolor.",
	},
	{
		title: "Kohalik karujüri",
		date: new Date(2018, 4, 3),
		images: [
			{ src: "/test.jpg", alt: "rand" },
			{ src: "/test.jpg", alt: "rand" },
			{ src: "/test.jpg", alt: "rand" },
		],
		text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Morbi porttitor sapien non bibendum tincidunt. Vivamus tincidunt lorem dui, ac fringilla dolor porta quis. Vivamus nec tortor ac lorem molestie congue. Mauris ullamcorper id sapien id mattis. In at iaculis dolor.",
	},
	{
		title: "Uudis",
		date: new Date(2017, 3, 1),
		images: [{ src: "/test.jpg", alt: "poisid" }],
		text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Morbi porttitor sapien non bibendum tincidunt.",
	}
];

export async function fetchAvailablePuppies(): Promise<Litter[]> {
	const litters = LITTERS.map((l) => {
		const available = l.puppies.filter(
			(p) => p.availability !== Availability.UNAVAILABLE,
		);
		return { ...l, puppies: available };
	});

	return new Promise((resolve) => {
		setTimeout(() => resolve(litters), 3000);
	});
}

export async function fetchAvailableBreedPuppies(breed: Breed): Promise<Litter[]> {
	const litters = LITTERS.filter((l) => l.breed === breed).map((l) => {
		const available = l.puppies.filter(
			(p) => p.availability !== Availability.UNAVAILABLE,
		);
		return { ...l, puppies: available };
	});

	return new Promise((resolve) => {
		setTimeout(() => resolve(litters), 3000);
	});
}

export async function fetchBreedDogs(
	breed: Breed,
	gender?: Gender,
): Promise<DogPreview[]> {
	const dogs = PREVIEWS.filter((d) => d.breed === breed)
		.filter((d) => d.alive)
		.filter((d) => d.gender === gender);

	return new Promise((resolve) => {
		setTimeout(() => resolve(dogs), 3000);
	});
}

export async function fetchBreedDogsRetired(breed: Breed): Promise<DogPreview[]> {
	const dogs = PREVIEWS.filter((d) => d.breed === breed).filter((d) => !d.alive);

	return new Promise((resolve) => {
		setTimeout(() => resolve(dogs), 2000);
	});
}

export async function fetchDog(name: string): Promise<Dog> {
	const dog = name === "Salsa" ? SALSA : KATJA;

	return new Promise((resolve) => {
		setTimeout(() => resolve(dog), 3000);
	});
}

export async function fetchFamily(): Promise<Family> {
	return new Promise((resolve) => {
		setTimeout(() => resolve(FAMILY), 5000);
	});
}

export async function fetchNames(): Promise<string[]> {
	const names = LITTERS.map((l) => l.name);

	return new Promise((resolve) => {
		setTimeout(() => resolve(names), 1000);
	});
}

export async function fetchLitter(name: string): Promise<Litter | undefined> {
	const match = LITTERS.find((l) => l.name === name);
	return new Promise((resolve) => {
		setTimeout(() => resolve(match), 1000);
	});
}

export async function fetchNews(from: Date, count = 5): Promise<NewsPiece[]> {
	const news = NEWS.filter(n => n.date < from).slice(0, count)

	return new Promise((resolve) => {
		setTimeout(() => resolve(news), 3000);
	});
}
