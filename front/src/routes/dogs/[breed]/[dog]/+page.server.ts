import { Breed, Gender, type Dog, type Family } from "$lib/types";
import type { PageServerLoad } from "./$types";

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

export const load: PageServerLoad = async ({ params }) => {
    const name = params.dog;
    return {
        dog: fetchDog(name),
        tree: fetchFamily(),
    };
};

async function fetchDog(name: string): Promise<Dog> {
    const dog = name === "Salsa" ? SALSA : KATJA;

    return new Promise((resolve) => {
        setTimeout(() => resolve(dog), 3000);
    });
}

async function fetchFamily(): Promise<Family> {
    return new Promise((resolve) => {
        setTimeout(() => resolve(FAMILY), 5000);
    });
}
