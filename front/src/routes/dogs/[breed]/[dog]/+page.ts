import { Breed, Gender, type Dog } from "$lib/types";
import type { PageLoad } from "./$types";

export const load: PageLoad = () => {
    const dog: Dog = {
        thumbnail: { src: "/rand.webp", alt: "rand" },
        images: [
            { src: "/rand.webp", alt: "rand" },
            { src: "/rand.webp", alt: "rand" },
            { src: "/rand.webp", alt: "rand" },
            { src: "/rand.webp", alt: "rand" },
            { src: "/rand.webp", alt: "rand" },
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
        health: ["HD-AA", "ED-00", "MDR1 +/+", "CEA carrier", "HC clear by parentage", "PRA clear by parentage", "Eyes clear"],
    };

    return { dog };
};
