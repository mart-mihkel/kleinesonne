import { nanoid } from "nanoid";
import { parseDate } from "$lib";
import { Availability, Breed, Gender } from "$lib/enums";
import type { Article, Dog, Family, Image, Litter, Puppy } from "$lib/types";
import { API_UPLOADS } from "./api/uploads";

type ImagePrefix = "news" | "dogs" | "litters" | "";

async function mapImages(
    images: File[],
    prefix: ImagePrefix = "",
): Promise<Image[]> {
    const promises = (images as File[])
        .filter((f) => f.size !== 0)
        .map(async (f) => {
            const name = `${API_UPLOADS}/${prefix ? prefix + "-" : ""}${nanoid()}.avif`;
            const buf = await f.arrayBuffer();
            const bytes = new Uint8Array(buf);
            const ascii = [...bytes]
                .map((b) => String.fromCharCode(b))
                .join("");
            const b64 = btoa(ascii);
            return { name, b64 };
        });

    return Promise.all(promises);
}

export async function formArticle(
    data: FormData,
): Promise<[Article | undefined, Image[]]> {
    const id = data.get("id");
    const title = data.get("title");
    const date = data.get("date");
    const text = data.get("text");
    const images = data.getAll("images");

    if (!id || !title || !date || !text || !images) {
        return [undefined, []];
    }

    const files = await mapImages(images as File[], "news");

    return [
        {
            id: Number(id),
            title: title as string,
            date: parseDate(date as string),
            text: text as string,
            images: files.map((f) => f.name),
        },
        files,
    ];
}

export async function formDog(
    data: FormData,
): Promise<[Dog | undefined, Image[]]> {
    const id = data.get("id");
    const name = data.get("name");
    const nickname = data.get("nickname");
    const father = data.get("father");
    const mother = data.get("mother");
    const thumbnail = data.get("thumbnail");
    const dob = data.get("dob");
    const breed = data.get("breed");
    const gender = data.get("gender");
    const alive = data.get("alive");
    const images = data.getAll("images");
    const titles = data.get("titles");
    const health = data.get("health");

    if (
        !id ||
        !name ||
        !nickname ||
        !father ||
        !mother ||
        !thumbnail ||
        !dob ||
        !breed ||
        !gender ||
        !alive ||
        !images
    ) {
        return [undefined, []];
    }

    const file = await mapImages([thumbnail as File]);
    const files = await mapImages(images as File[], "dogs");

    return [
        {
            id: Number(id),
            name: name as string,
            nickname: nickname as string,
            father: father as string,
            mother: mother as string,
            thumbnail: file[0]?.name,
            dob: parseDate(dob as string),
            breed: breed as Breed,
            gender: gender as Gender,
            alive: alive === "true",
            images: files.map((f) => f.name),
            titles: titles ? (titles as string).split(",") : [],
            health: health ? (health as string).split(",") : [],
        },
        file.concat(files),
    ];
}

export async function formFamily(data: FormData): Promise<Family | undefined> {
    const name = data.get("name");
    const father = data.get("father");
    const mother = data.get("mother");

    if (!name) {
        return undefined;
    }

    return {
        name: name as string,
        father: father ? { name: father as string } : undefined,
        mother: mother ? { name: mother as string } : undefined,
    };
}

export async function formLitter(
    data: FormData,
): Promise<[Litter | undefined, Image[]]> {
    const id = data.get("id");
    const name = data.get("name");
    const dob = data.get("dob");
    const breed = data.get("breed");
    const parents_image = data.get("parents_image");
    const images = data.getAll("images");

    if (!id || !name || !dob || !breed || !images) {
        return [undefined, []];
    }

    const file = await mapImages([parents_image as File]);
    const files = await mapImages(images as File[], "litters");

    return [
        {
            id: Number(id),
            name: name as string,
            dob: parseDate(dob as string),
            breed: breed as Breed,
            parents_image: file[0]?.name,
            images: files.map((f) => f.name),
        },
        file.concat(files),
    ];
}

export async function formPuppy(
    data: FormData,
): Promise<[Puppy | undefined, Image[]]> {
    const id = data.get("id");
    const litter_id = data.get("litter_id");
    const name = data.get("name");
    const gender = data.get("gender");
    const availability = data.get("availability");
    const image = data.get("image");

    if (!id || !name || !gender || !availability) {
        return [undefined, []];
    }

    const file = await mapImages([image as File]);

    return [
        {
            id: Number(id),
            litter_id: Number(litter_id),
            name: name as string,
            gender: gender as Gender,
            availability: availability as Availability,
            image: file[0]?.name,
        },
        file,
    ];
}
