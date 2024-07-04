import { Breed, Gender, type Dog, type Family } from "$lib/types";
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

    const family: Family = {
        name: "Kleine Sonne Awesome Huntress",
        father: {
            name: "Snowbelts Winning Ticket",
            father: {
                name: "Dreamstreets Season Ticket",
                father: { name: "Briarbrooks Valedictorian" },
                mother: { name: "Mysharas Shameless" }
            },
            mother: {
                name: "Mysharas Oh What a Night",
                father: { name: "Briarbrooks Copyright" },
                mother: { name: "Briarbrooks Finerthingsinlife" },
            }
        },
        mother: {
            name: "Bleuroyal Creamy Vodkanilla",
            father: {
                name: "Thornapple Uncle Sam Wants You",
                father: { name: "Thornapple Single Barrel" },
                mother: { name: "Thornapple Russian To a Party" }
            },
            mother: {
                name: "Thornapple Peachy Keen",
                father: { name: "Thornapple Hot Temptation" },
                mother: { name: "Thornapple Pieces Of My Hearth" }
            }
        },
    };

    return { dog, family };
};
