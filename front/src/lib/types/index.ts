export type LongBreed =
    | "Australian Shepherd"
    | "Bernese Mountain Dog"
    | "Entlebuch Cattle Dog";

export type DogPreview = {
    thumbnail: string;
    name: string;
    nickname: string;
    breed: Breed;
    gender: Gender;
    alive: boolean;
};

export type Dog = {
    thumbnail: string;
    images: string[];
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
};

export type Family = {
    name: string;
    father?: Family;
    mother?: Family;
};

export type Puppy = {
    image: string;
    name: string;
    gender: Gender;
    availability: Availability;
};

export type Litter = {
    parents: string;
    name: string;
    breed: Breed;
    images: string[];
    puppies: Puppy[];
};

export type NewsPiece = {
    title: string;
    date: Date;
    images: string[];
    text: string;
};

export type ModalDispatch = {
    select: string;
    delete: string;
}

export enum Gender {
    MALE = "male",
    FEMALE = "female",
}

export enum Breed {
    AUSTRALIAN = "aus",
    BERNESE = "ber",
    ENTLEBUCH = "ent",
}

export enum Availability {
    AVAILABLE,
    UNAVAILABLE,
    CO_OWNERSHIP,
}
