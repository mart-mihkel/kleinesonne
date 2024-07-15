import { fail } from "@sveltejs/kit";
import type { Actions, PageServerLoad } from "./$types";
import {
    authenticate,
    login,
    updateArticle,
    updatePuppy,
    updateLitter,
    updateDog,
    uploadArticle,
    uploadDog,
    uploadLitter,
    uploadPuppy,
    uploadImages,
} from "$lib/api";
import { parseDate } from "$lib";
import {
    Availability,
    Breed,
    Gender,
    type Article,
    type Dog,
    type Image,
    type Litter,
    type Puppy,
} from "$lib/types";
import { API_UPLOADS } from "$lib/api/uploads";

export const load: PageServerLoad = async ({ cookies }) => {
    const jwt = cookies.get("jwt");
    const authenticated = jwt !== undefined ? await authenticate(jwt) : false;
    return { jwt: authenticated ? jwt : undefined };
};

export const actions: Actions = {
    login: async ({ request, cookies }) => {
        const data = await request.formData();

        const user = data.get("user");
        const secret = data.get("secret");

        if (!user || !secret) {
            return fail(400, { error: "Missing credentials" });
        }

        const token = await login(user as string, secret as string);
        cookies.set("jwt", `Bearer ${token.token}`, { path: "/admin" });
    },
    newsCreate: async ({ request, cookies }) => {
        const jwt = cookies.get("jwt");
        if (jwt === undefined) {
            return fail(401, { error: "Unauthorized" });
        }

        const data = await request.formData();
        const [article, images] = await formArticle(data);
        if (article === undefined) {
            return fail(400, { error: "Incomplete form" });
        }

        uploadImages(images, jwt);
        uploadArticle(article, jwt);
    },
    newsUpdate: async ({ request, cookies }) => {
        const jwt = cookies.get("jwt");
        if (jwt === undefined) {
            return fail(401, { error: "Unauthorized" });
        }

        const data = await request.formData();
        const [article, images] = await formArticle(data);
        if (article === undefined) {
            return fail(400, { error: "Incomplete form" });
        }

        uploadImages(images, jwt);
        updateArticle(article, jwt);
    },
    dogCreate: async ({ request, cookies }) => {
        const jwt = cookies.get("jwt");
        if (jwt === undefined) {
            return fail(401, { error: "Unauthorized" });
        }

        const data = await request.formData();
        const [dog, images] = await formDog(data);
        if (dog === undefined) {
            return fail(400, { error: "Incomplete form" });
        }

        uploadImages(images, jwt);
        uploadDog(dog, jwt);
    },
    dogUpdate: async ({ request, cookies }) => {
        const jwt = cookies.get("jwt");
        if (jwt === undefined) {
            return fail(401, { error: "Unauthorized" });
        }

        const data = await request.formData();
        const [dog, images] = await formDog(data);
        if (dog === undefined) {
            return fail(400, { error: "Incomplete form" });
        }

        uploadImages(images, jwt);
        updateDog(dog, jwt);
    },
    litterCreate: async ({ request, cookies }) => {
        const jwt = cookies.get("jwt");
        if (jwt === undefined) {
            return fail(401, { error: "Unauthorized" });
        }

        const data = await request.formData();
        const [litter, images] = await formLitter(data);
        if (litter === undefined) {
            return fail(400, { error: "Incomplete form" });
        }

        uploadImages(images, jwt);
        uploadLitter(litter, jwt);
    },
    litterUpdate: async ({ request, cookies }) => {
        const jwt = cookies.get("jwt");
        if (jwt === undefined) {
            return fail(401, { error: "Unauthorized" });
        }

        const data = await request.formData();
        const [litter, images] = await formLitter(data);
        if (litter === undefined) {
            return fail(400, { error: "Incomplete form" });
        }

        uploadImages(images, jwt);
        updateLitter(litter, jwt);
    },
    puppyCreate: async ({ request, cookies }) => {
        const jwt = cookies.get("jwt");
        if (jwt === undefined) {
            return fail(401, { error: "Unauthorized" });
        }

        const data = await request.formData();
        const [puppy, images] = await formPuppy(data);
        if (puppy === undefined) {
            return fail(400, { error: "Incomplete form" });
        }

        uploadImages(images, jwt);
        uploadPuppy(puppy, jwt);
    },
    puppyUpdate: async ({ request, cookies }) => {
        const jwt = cookies.get("jwt");
        if (jwt === undefined) {
            return fail(401, { error: "Unauthorized" });
        }

        const data = await request.formData();
        const [puppy, images] = await formPuppy(data);
        if (puppy === undefined) {
            return fail(400, { error: "Incomplete form" });
        }

        uploadImages(images, jwt);
        updatePuppy(puppy, jwt);
    },
};

async function mapImages(images: File[]): Promise<Image[]> {
    const promises = (images as File[])
        .filter((f) => f.size !== 0)
        .map(async (f) => {
            const name = API_UPLOADS + f.name;
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

async function formArticle(
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

    const files = await mapImages(images as File[]);

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

async function formDog(data: FormData): Promise<[Dog | undefined, Image[]]> {
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
        !images ||
        !titles ||
        !health
    ) {
        return [undefined, []];
    }

    const file = await mapImages([thumbnail as File]);
    const files = await mapImages(images as File[]);

    return [
        {
            id: Number(id),
            name: name as string,
            nickname: nickname as string,
            father: father as string,
            mother: mother as string,
            thumbnail: file[0].name,
            dob: parseDate(dob as string),
            breed: breed as Breed,
            gender: gender as Gender,
            alive: alive === "true",
            images: files.map((f) => f.name),
            titles: (titles as string).split(","),
            health: (health as string).split(","),
        },
        file.concat(files),
    ];
}

async function formLitter(
    data: FormData,
): Promise<[Litter | undefined, Image[]]> {
    const id = data.get("id");
    const name = data.get("name");
    const breed = data.get("breed");
    const parents_image = data.get("parents_image");
    const images = data.getAll("images");

    if (!id || !name || !breed || !parents_image || !images) {
        return [undefined, []];
    }

    const files = await mapImages(images as File[]);

    return [
        {
            id: Number(id),
            name: name as string,
            breed: breed as Breed,
            parents_image: "/img.jpg",
            images: files.map((f) => f.name),
        },
        files,
    ];
}

async function formPuppy(
    data: FormData,
): Promise<[Puppy | undefined, Image[]]> {
    const id = data.get("id");
    const litter_id = data.get("litter_id");
    const name = data.get("name");
    const gender = data.get("gender");
    const availability = data.get("availability");
    const image = data.get("image");

    if (!id || !name || !gender || !availability || !image) {
        return [undefined, []];
    }

    const files = await mapImages([image as File]);

    return [
        {
            id: Number(id),
            litter_id: Number(litter_id),
            name: name as string,
            gender: gender as Gender,
            availability: availability as Availability,
            image: files[0] ? files[0].name : "",
        },
        files,
    ];
}
