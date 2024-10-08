<script lang="ts">
    import Gallery from "../Gallery.svelte";
    import Modal from "../Modal.svelte";
    import type { Name } from "$lib/types";
    import { Breed } from "$lib/enums";
    import { enhance } from "$app/forms";
    import { onMount } from "svelte";
    import { formDate } from "$lib";
    import { resdata } from "$lib/api";
    import { deleteImage } from "$lib/api/uploads";
    import {
        fetchLitterNames,
        fetchLitter,
        deleteLitter,
        API_LITTER,
    } from "$lib/api/litter";

    export let jwt: string;

    const INITIAL_DATA = {
        id: -1,
        name: "",
        dob: "",
        breed: Breed.AUSTRALIAN,
        parents_image: <string[]>[],
        images: <string[]>[],
    };

    let form = { ...INITIAL_DATA };
    let modal = false;
    let promise: Promise<Name[] | undefined> | undefined = undefined;

    onMount(() => (promise = loadNames()));

    async function loadNames(): Promise<Name[] | undefined> {
        const res = await fetchLitterNames();
        const data = resdata(res);
        return data.data;
    }

    function reset() {
        form = { ...INITIAL_DATA };
    }

    async function select(e: CustomEvent<number>) {
        const res = await fetchLitter(e.detail);
        const data = resdata(res);

        if (data.error) {
            alert(`Getting litter failed: ${data.error}`);
            return;
        }

        const litter = data.data!;
        form = {
            ...litter,
            dob: formDate(litter.dob),
            parents_image: litter.parents_image ? [litter.parents_image] : [],
        };
    }

    async function del(e: CustomEvent<number>) {
        const { res } = await deleteLitter(e.detail, jwt);
        if (res.type === "error") {
            alert(`Deleting litter failed: ${res.error}`);
        }
    }

    async function delParents() {
        const { res } = await deleteImage(
            form.id,
            jwt,
            API_LITTER + "/delete/parents",
        );

        if (res.type === "error") {
            alert(`Deleting parents image failed: ${res.error}`);
        }
    }

    async function delImage(e: CustomEvent<string>) {
        const { res } = await deleteImage(
            form.id,
            jwt,
            API_LITTER + "/delete/image",
            e.detail,
        );

        if (res.type === "error") {
            alert(`Deleting image failed: ${res.error}`);
        }
    }
</script>

<div class="flex flex-col">
    <h3
        class="border-t border-black p-4 text-center text-2xl font-semibold dark:border-white"
    >
        Litters
    </h3>
    <form
        method="POST"
        id="litterform"
        class="flex flex-col"
        enctype="multipart/form-data"
        use:enhance
    >
        <label class="hidden">
            <input type="hidden" name="id" bind:value={form.id} />
        </label>
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Litter name</p>
            <input
                class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                type="text"
                name="name"
                required
                bind:value={form.name}
            />
        </label>
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Date of birth</p>
            <input
                class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                type="date"
                name="dob"
                required
                bind:value={form.dob}
            />
        </label>
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Breed</p>
            <select
                class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                name="breed"
                required
                bind:value={form.breed}
            >
                <option value={Breed.AUSTRALIAN}>{Breed.AUSTRALIAN}</option>
                <option value={Breed.ENTLEBUCH}>{Breed.ENTLEBUCH}</option>
                <option value={Breed.BERNESE}>{Breed.BERNESE}</option>
            </select>
        </label>
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Parents</p>
            <input class="w-2/3 p-2" type="file" name="parents_image" />
        </label>
        <Gallery bind:images={form.parents_image} admin on:image={delParents} />
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Images</p>
            <input
                class="w-2/3 p-2"
                type="file"
                name="images"
                multiple={true}
            />
        </label>
    </form>
    <Gallery bind:images={form.images} admin on:image={delImage} />
    <div class="flex flex-row flex-wrap justify-center gap-4 p-4">
        <button
            class="h-12 w-48 rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
            on:click|preventDefault={() => (modal = true)}
        >
            {#if promise === undefined}
                <p>Loading names...</p>
            {:else}
                {#await promise}
                    <p>Loading names...</p>
                {:then titles}
                    {#if titles === undefined}
                        <p>Server error</p>
                    {:else}
                        <p>Select</p>
                        <Modal
                            items={titles}
                            bind:open={modal}
                            on:select={select}
                            on:delete={del}
                        />
                    {/if}
                {:catch}
                    <p>Server error</p>
                {/await}
            {/if}
        </button>
        <button
            class="h-12 w-48 rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
            on:click|preventDefault={reset}
        >
            Reset form
        </button>
        <button
            class="h-12 w-48 rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
            form="litterform"
            formaction="?/litterCreate"
        >
            Create
        </button>
        <button
            class="h-12 w-48 rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
            form="litterform"
            formaction="?/litterUpdate"
        >
            Update
        </button>
    </div>
</div>
