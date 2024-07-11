export type LongBreed =
    | "Australian Shepherd"
    | "Bernese Mountain Dog"
    | "Entlebuch Cattle Dog";

export type Login = {
    user: string;
    secret: string;
};

export type DogPreview = {
    id: string;
    name: string;
    nickname: string;
    thumbnail: string;
    breed: Breed;
    gender: Gender;
    alive: boolean;
};

export type Dog = {
    id: number;
    name: string;
    nickname: string;
    father: string;
    mother: string;
    thumbnail: string;
    dob: Date;
    breed: Breed;
    gender: Gender;
    alive: boolean;
    images: string[];
    health: string[];
    titles: string[];
};

export type Family = {
    name: string;
    father?: Family;
    mother?: Family;
};

export type Puppy = {
    id: number;
    name: string;
    image: string;
    gender: Gender;
    availability: Availability;
};

export type Litter = {
    id: number;
    name: string;
    parents_image: string;
    breed: Breed;
    images: string[];
    puppies: Puppy[];
};

export type Article = {
    id: number;
    title: string;
    text: string;
    date: Date;
    images: string[];
};

export type ModalDispatch = {
    select: string;
    delete: string;
};

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
    AVAILABLE = "available",
    UNAVAILABLE = "unavailable",
    CO_OWNERSHIP = "co",
}
