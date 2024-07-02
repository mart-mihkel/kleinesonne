export type LongBreed =
    | "Australian Shepherd"
    | "Bernese Mountain Dog"
    | "Entlebucher Mountain Dog";

export enum Gender {
    MALE,
    FEMALE,
}

export enum Breed {
    AUSTRALIAN = "aus",
    BERNESE = "bern",
    ENTLEBUCHER = "entle",
}

export enum Availability {
    AVAILABLE,
    UNAVAILABLE,
    CO_OWNERSHIP,
}

export interface DogPreview {
    thumbnail: Image;
    name: string;
    nickname: string;
    breed: Breed;
    gender: Gender;
    alive: boolean;
}

export interface Dog {
    thumbnail: Image;
    images: Image[];
    name: string;
    nickname: string;
    breed: Breed;
    gender: Gender;
    alive: boolean;
    dob: Date;
    dod?: Date;
}

export interface Puppy {
    image: Image;
    name: string;
    gender: Gender;
    availability: Availability;
}

export interface Litter {
    parents: Image;
    father: string;
    mother: string;
    name: string;
    breed: Breed;
    puppies: Array<Puppy>;
}

export interface Image {
    src: string;
    alt: string;
}
