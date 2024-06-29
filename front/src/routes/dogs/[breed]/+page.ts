import { Breed, Gender, type DogPreview } from "$lib/types";
import type { PageLoad } from "./$types";

export const load: PageLoad = () => {
    const dogs: DogPreview[] = [
        {
            thumbnail: { src: "/rand.webp", alt: "rand" },
            nickname: "Salsa",
            name: "Seventy Seven Spicy Salsa",
            breed: Breed.AUSTRALIAN,
            gender: Gender.FEMALE,
            alive: true,
        },
        {
            thumbnail: { src: "/kohver.webp", alt: "kohver" },
            nickname: "Katja",
            name: "Korolevstvo Gornih Psov Okatava",
            breed: Breed.ENTLEBUCHER,
            gender: Gender.FEMALE,
            alive: true,
        },
    ];

    return { dogs };
};
