import { Breed, Gender, type DogPreview } from "$lib/types";
import type { PageServerLoad } from "./$types";

const DOGS: DogPreview[] = [
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

export const load: PageServerLoad = async ({ params }) => {
    const breed = params.breed as Breed;
    return {
        male: fetchBreedDogs(breed, Gender.MALE),
        female: fetchBreedDogs(breed, Gender.FEMALE),
        retired: fetchBreedDogsRetired(breed),
    };
};

async function fetchBreedDogs(
    breed: Breed,
    gender?: Gender,
): Promise<DogPreview[]> {
    const dogs = DOGS.filter((d) => d.breed === breed)
        .filter((d) => d.alive)
        .filter((d) => d.gender === gender);

    return new Promise((resolve) => {
        setTimeout(() => resolve(dogs), 3000);
    });
}

async function fetchBreedDogsRetired(breed: Breed): Promise<DogPreview[]> {
    const dogs = DOGS.filter((d) => d.breed === breed).filter((d) => !d.alive);

    return new Promise((resolve) => {
        setTimeout(() => resolve(dogs), 2000);
    });
}
