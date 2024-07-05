import { Gender, Breed, type Litter, Availability } from "$lib/types";
import type { PageServerLoad } from "./$types";

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

export const load: PageServerLoad = ({ params }) => {
    return { litters: fetchAvailableBreedPuppies(params.breed as Breed) };
};

async function fetchAvailableBreedPuppies(breed: Breed): Promise<Litter[]> {
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
