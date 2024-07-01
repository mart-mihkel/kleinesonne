import { Gender, Breed, type Litter } from "$lib/types";
import type { PageLoad } from "./$types";

export const load: PageLoad = ({ params }) => {
    const litters: Litter[] = [
        {
            name: "x",
            parents: { src: "/parents.webp", alt: "parents" },
            father: "Czech Made Swiss Tricolor",
            mother: "Kleine Sonne Be The Shine of My Day",
            breed: Breed.ENTLEBUCHER,
            puppies: [
                {
                    image: { src: "/usin.webp", alt: "usin" },
                    name: "Kleine Sonne Golf in Leuk",
                    gender: Gender.MALE,
                    available: true,
                },
                {
                    image: { src: "/usin.webp", alt: "usin" },
                    name: "Kleine Sonne Golf in Davos",
                    gender: Gender.MALE,
                    available: true,
                },
                {
                    image: { src: "/usin.webp", alt: "usin" },
                    name: "Kleine Sonne Golf in Erlen",
                    gender: Gender.FEMALE,
                    available: true,
                },
            ],
        },
    ];

    if (params.breed !== Breed.ENTLEBUCHER) {
        return { litters: [] };
    }

    return { litters };
};
