import { Breed, Gender, type Dog } from "$lib/types";
import type { PageLoad } from "./$types";

export const load: PageLoad = () => {
    const dog: Dog = {
        thumbnail: { src: "/rand.webp", alt: "rand" },
        images: [{ src: "/rand.webp", alt: "rand" }],
        name: "Seventy Seven Spicy Salsa",
        nickname: "Salsa",
        breed: Breed.AUSTRALIAN,
        gender: Gender.FEMALE,
        alive: true,
        dob: new Date(2016, 4, 22),
    };

    return { dog };
};
