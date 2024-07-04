import { Gender, Breed, type Litter, Availability } from "$lib/types";
import type { PageServerLoad } from "./$types";

const LITTERS: Litter[] = [
    {
        name: "Entlebuch Marakratid",
        parents: { src: "/parents.webp", alt: "parents" },
        breed: Breed.ENTLEBUCH,
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
];

export const load: PageServerLoad = ({ params }) => {
    return { litters: fetchAvailableBreedPuppies(params.breed as Breed) };
};

async function fetchAvailableBreedPuppies(breed: Breed): Promise<Litter[]> {
    const litters = LITTERS
        .filter(l => l.breed === breed)
        .map(l => {
            const available = l.puppies.filter(p => p.availability !== Availability.UNAVAILABLE);
            return { ...l, puppies: available };
        });

    return new Promise(resolve => {
        setTimeout(() => resolve(litters), 3000);
    });
}
