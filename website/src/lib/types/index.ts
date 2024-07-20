import type { Availability, Breed, Gender } from "$lib/enums";

export type LongBreed =
    | "Australian Shepherd"
    | "Bernese Mountain Dog"
    | "Entlebuch Cattle Dog";

export type Login = {
    user: string;
    secret: string;
};

export type Dog = {
    id: number;
    name: string;
    nickname: string;
    father: string;
    mother: string;
    thumbnail: string | undefined;
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
    image: string | undefined;
    gender: Gender;
    availability: Availability;
};

export type Litter = {
    id: number;
    name: string;
    parents_image: string | undefined;
    breed: Breed;
    images: string[];
};

export type LitterWithPuppies = {
    id: number;
    name: string;
    parents_image: string | undefined;
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
    image: string;
};

export type Name = {
    id: number;
    name: string;
};

export type Title = {
    id: number;
    title: string;
};

export type Image = {
    name: string;
    b64: string;
};

export type SsrFetch = (input: string, init: RequestInit) => Promise<Response>;

export type ApiResponse<D> = {
    res:
        | { type: "success" }
        | { type: "error"; error: string }
        | { type: "token"; token: string }
        | { type: "data"; data: D };
};

export type ResponseData<D> = { error?: string; data?: D };
