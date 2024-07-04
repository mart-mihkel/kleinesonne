export type LongBreed =
    | "Australian Shepherd"
    | "Bernese Mountain Dog"
    | "Entlebuch Cattle Dog";

export type DogPreview = {
    thumbnail: Image;
    name: string;
    nickname: string;
    breed: Breed;
    gender: Gender;
    alive: boolean;
}

export type Dog = {
    thumbnail: Image;
    images: Image[];
    father: string;
    mother: string;
    name: string;
    nickname: string;
    breed: Breed;
    gender: Gender;
    alive: boolean;
    awards: string[];
    health: string[];
    dob: Date;
    dod?: Date;
}

export type Family = {
    name: string;
    father?: Family;
    mother?: Family;
};

export type Puppy = {
    image: Image;
    name: string;
    gender: Gender;
    availability: Availability;
}

export type Litter = {
    parents: Image;
    name: string;
    breed: Breed;
    puppies: Array<Puppy>;
}

export type NewsPiece = {
    title: string;
    date: Date;
    images: Image[];
    text: string;
}

export type Image = {
    src: string;
    alt: string;
}

export enum Gender {
    MALE,
    FEMALE,
}

export enum Breed {
    AUSTRALIAN = "australian",
    BERNESE = "bernese",
    ENTLEBUCH = "entlebuch",
}

export enum Availability {
    AVAILABLE,
    UNAVAILABLE,
    CO_OWNERSHIP,
}
