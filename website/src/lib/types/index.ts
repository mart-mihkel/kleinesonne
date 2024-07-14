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
    dob: number;
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
    litter_id: number;
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
};

export type LitterWithPuppies = {
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
    date: number;
    images: string[];
};

export type ModalDispatch = {
    select: number;
    delete: number;
};

export type Id = {
    id: number;
};

export type Name = {
    id: number;
    name: string;
};

export type Title = {
    id: number;
    title: string;
};

export enum Gender {
    MALE = "Male",
    FEMALE = "Female",
}

export enum Breed {
    AUSTRALIAN = "Australian",
    BERNESE = "Bernese",
    ENTLEBUCH = "Entlebuch",
}

export enum Availability {
    AVAILABLE = "Available",
    UNAVAILABLE = "Unavailable",
    CO_OWNERSHIP = "Co",
}
