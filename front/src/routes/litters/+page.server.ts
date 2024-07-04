import { Gender, Breed, type Litter, Availability } from "$lib/types";
import type { PageServerLoad } from "./$types";

const LITTERS: Litter[] = [
    {
        name: "Entlebuch Marakratid",
        parents: { src: "/parents.webp", alt: "parents" },
        breed: Breed.ENTLEBUCH,
        images: [],
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
        images: [
            { src: "/rand.webp", alt: "rand" },
            { src: "/rand.webp", alt: "rand" },
            { src: "/rand.webp", alt: "rand" },
            { src: "/rand.webp", alt: "rand" },
            { src: "/rand.webp", alt: "rand" },
            { src: "/rand.webp", alt: "rand" },
            { src: "/rand.webp", alt: "rand" },
        ],
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
    {
        name: "Olematud Olendid",
        parents: { src: "", alt: "pole olemas" },
        breed: Breed.AUSTRALIAN,
        images: [],
        puppies: [
            {
                image: { src: "", alt: "mitte midagi" },
                name: "Kleine Sonne Not Real",
                gender: Gender.MALE,
                availability: Availability.CO_OWNERSHIP,
            },
        ],
    },
];

export const load: PageServerLoad = () => {
    return {
        names: fetchNames(),
    };
};

async function fetchNames(): Promise<string[]> {
    const names = LITTERS.map((l) => l.name);

    return new Promise((resolve) => {
        setTimeout(() => resolve(names), 1000);
    });
}
