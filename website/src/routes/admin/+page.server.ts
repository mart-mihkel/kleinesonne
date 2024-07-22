import { fail } from "@sveltejs/kit";
import type { Actions, PageServerLoad } from "./$types";
import {
    formArticle,
    formDog,
    formFamily,
    formLitter,
    formPuppy,
} from "$lib/forms";
import { UNEXPECTED_ERROR } from "$lib/api";
import { authenticate, login } from "$lib/server/auth";
import { updateArticle, uploadArticle } from "$lib/server/news";
import { uploadImages } from "$lib/server/uploads";
import { updateDog, updateFamily, uploadDog } from "$lib/server/dog";
import { updateLitter, uploadLitter } from "$lib/server/litter";
import { updatePuppy, uploadPuppy } from "$lib/server/puppy";

export const load: PageServerLoad = async ({ fetch, cookies }) => {
    const error = { jwt: undefined };
    const jwt = cookies.get("jwt");

    if (jwt === undefined) {
        return error;
    }

    const { res } = await authenticate(fetch, jwt);
    return res.type === "success" ? { jwt } : error;
};

export const actions: Actions = {
    login: async ({ fetch, request, cookies }) => {
        const data = await request.formData();

        const user = data.get("user");
        const secret = data.get("secret");

        if (!user || !secret) {
            return fail(400, { error: "Missing credentials" });
        }

        const { res } = await login(fetch, user as string, secret as string);

        if (res.type === "error") {
            return fail(500, { error: res.error });
        }

        if (res.type !== "token") {
            return fail(500, UNEXPECTED_ERROR);
        }

        cookies.set("jwt", `Bearer ${res.token}`, { path: "/admin" });
        return { success: true };
    },
    newsCreate: async ({ fetch, request, cookies }) => {
        const jwt = cookies.get("jwt");
        if (jwt === undefined) {
            return fail(401, { error: "Unauthorized" });
        }

        const data = await request.formData();
        const [article, images] = await formArticle(data);
        if (article === undefined) {
            return fail(400, { error: "Incomplete form" });
        }

        const { res } = await uploadArticle(fetch, article, jwt);
        if (res.type === "error") {
            return fail(500, { error: res.error });
        }

        const img = await uploadImages(fetch, images, jwt);
        if (img.res.type === "error") {
            return fail(500, { error: img.res.error });
        }

        return { success: true };
    },
    newsUpdate: async ({ fetch, request, cookies }) => {
        const jwt = cookies.get("jwt");
        if (jwt === undefined) {
            return fail(401, { error: "Unauthorized" });
        }

        const data = await request.formData();
        const [article, images] = await formArticle(data);
        if (article === undefined) {
            return fail(400, { error: "Incomplete form" });
        }

        const { res } = await updateArticle(fetch, article, jwt);
        if (res.type === "error") {
            return fail(500, { error: res.error });
        }

        const img = await uploadImages(fetch, images, jwt);
        if (img.res.type === "error") {
            return fail(500, { error: img.res.error });
        }

        return { success: true };
    },
    dogCreate: async ({ fetch, request, cookies }) => {
        const jwt = cookies.get("jwt");
        if (jwt === undefined) {
            return fail(401, { error: "Unauthorized" });
        }

        const data = await request.formData();
        const [dog, images] = await formDog(data);
        if (dog === undefined) {
            return fail(400, { error: "Incomplete form" });
        }

        const { res } = await uploadDog(fetch, dog, jwt);
        if (res.type === "error") {
            return fail(500, { error: res.error });
        }

        const img = await uploadImages(fetch, images, jwt);
        if (img.res.type === "error") {
            return fail(500, { error: img.res.error });
        }

        const fam = await updateFamily(dog.name, dog.father, dog.mother);
        if (fam.res.type === "error") {
            return fail(500, { error: fam.res.error });
        }

        return { success: true };
    },
    dogUpdate: async ({ fetch, request, cookies }) => {
        const jwt = cookies.get("jwt");
        if (jwt === undefined) {
            return fail(401, { error: "Unauthorized" });
        }

        const data = await request.formData();
        const [dog, images] = await formDog(data);
        if (dog === undefined) {
            return fail(400, { error: "Incomplete form" });
        }

        const { res } = await updateDog(fetch, dog, jwt);
        if (res.type === "error") {
            return fail(500, { error: res.error });
        }

        const img = await uploadImages(fetch, images, jwt);
        if (img.res.type === "error") {
            return fail(500, { error: img.res.error });
        }

        const fam = await updateFamily(dog.name, dog.father, dog.mother);
        if (fam.res.type === "error") {
            return fail(500, { error: fam.res.error });
        }

        return { success: true };
    },
    familyUpdate: async ({ request, cookies }) => {
        const jwt = cookies.get("jwt");
        if (jwt === undefined) {
            return fail(401, { error: "Unauthorized" });
        }

        const data = await request.formData();
        const family = await formFamily(data);
        if (family === undefined) {
            return fail(400, { error: "Incomplete form" });
        }

        const { res } = await updateFamily(
            family.name,
            family.father?.name,
            family.mother?.name,
        );

        if (res.type === "error") {
            return fail(500, { error: res.error });
        }

        return { success: true };
    },

    litterCreate: async ({ fetch, request, cookies }) => {
        const jwt = cookies.get("jwt");
        if (jwt === undefined) {
            return fail(401, { error: "Unauthorized" });
        }

        const data = await request.formData();
        const [litter, images] = await formLitter(data);
        if (litter === undefined) {
            return fail(400, { error: "Incomplete form" });
        }

        const { res } = await uploadLitter(fetch, litter, jwt);
        if (res.type === "error") {
            return fail(500, { error: res.error });
        }

        const img = await uploadImages(fetch, images, jwt);
        if (img.res.type === "error") {
            return fail(500, { error: img.res.error });
        }

        return { success: true };
    },
    litterUpdate: async ({ fetch, request, cookies }) => {
        const jwt = cookies.get("jwt");
        if (jwt === undefined) {
            return fail(401, { error: "Unauthorized" });
        }

        const data = await request.formData();
        const [litter, images] = await formLitter(data);
        if (litter === undefined) {
            return fail(400, { error: "Incomplete form" });
        }

        const { res } = await updateLitter(fetch, litter, jwt);
        if (res.type === "error") {
            return fail(500, { error: res.error });
        }

        const img = await uploadImages(fetch, images, jwt);
        if (img.res.type === "error") {
            return fail(500, { error: img.res.error });
        }

        return { success: true };
    },
    puppyCreate: async ({ fetch, request, cookies }) => {
        const jwt = cookies.get("jwt");
        if (jwt === undefined) {
            return fail(401, { error: "Unauthorized" });
        }

        const data = await request.formData();
        const [puppy, images] = await formPuppy(data);
        if (puppy === undefined) {
            return fail(400, { error: "Incomplete form" });
        }

        const { res } = await uploadPuppy(fetch, puppy, jwt);
        if (res.type === "error") {
            return fail(500, { error: res.error });
        }

        const img = await uploadImages(fetch, images, jwt);
        if (img.res.type === "error") {
            return fail(500, { error: img.res.error });
        }

        return { success: true };
    },
    puppyUpdate: async ({ fetch, request, cookies }) => {
        const jwt = cookies.get("jwt");
        if (jwt === undefined) {
            return fail(401, { error: "Unauthorized" });
        }

        const data = await request.formData();
        const [puppy, images] = await formPuppy(data);
        if (puppy === undefined) {
            return fail(400, { error: "Incomplete form" });
        }

        const { res } = await updatePuppy(fetch, puppy, jwt);
        if (res.type === "error") {
            return fail(500, { error: res.error });
        }

        const img = await uploadImages(fetch, images, jwt);
        if (img.res.type === "error") {
            return fail(500, { error: img.res.error });
        }

        return { success: true };
    },
};
