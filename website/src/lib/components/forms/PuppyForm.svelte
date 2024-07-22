<script lang="ts">
    import Gallery from "../Gallery.svelte";
    import Modal from "../Modal.svelte";
    import type { Name } from "$lib/types";
    import { Availability, Gender } from "$lib/enums";
    import { enhance } from "$app/forms";
    import { onMount } from "svelte";
    import { resdata } from "$lib/api";
    import { fetchLitterNames, deleteLitter } from "$lib/api/litter";
    import { deleteImage } from "$lib/api/uploads";
    import {
        fetchPuppyNames,
        fetchPuppy,
        deletePuppy,
        API_PUPPY,
    } from "$lib/api/puppy";

    export let jwt: string;

    const INITIAL_DATA = {
        id: -1,
        litter_id: -1,
        name: "",
        gender: Gender.MALE,
        availability: Availability.AVAILABLE,
        image: <string[]>[],
    };

    let form = { ...INITIAL_DATA };

    let litterm = false;
    let puppym = false;

    let litterp: Promise<Name[] | undefined> | undefined = undefined;
    let namep: Promise<Name[] | undefined> | undefined = undefined;

    onMount(() => (litterp = loadLitters()));

    async function loadLitters(): Promise<Name[] | undefined> {
        const res = await fetchLitterNames();
        const data = resdata(res);
        return data.data;
    }

    async function loadNames(id: number): Promise<Name[] | undefined> {
        const res = await fetchPuppyNames(id);
        const data = resdata(res);
        return data.data;
    }

    function reset() {
        form = { ...INITIAL_DATA };
    }

    async function select(e: CustomEvent<number>) {
        const res = await fetchPuppy(e.detail);
        const data = resdata(res);

        if (data.error) {
            alert(`Getting puppy failed: ${data.error}`);
            return;
        }

        const puppy = data.data!;
        form = { ...puppy, image: puppy.image ? [puppy.image] : [] };
    }

    async function del(e: CustomEvent<number>) {
        const { res } = await deletePuppy(e.detail, jwt);
        if (res.type === "error") {
            alert(`Deleting puppy failed: ${res.error}`);
        }
    }

    function selectLitter(e: CustomEvent<number>) {
        const id = e.detail;
        form.litter_id = id;
        namep = loadNames(id);
    }

    async function delLitter(e: CustomEvent<number>) {
        const { res } = await deleteLitter(e.detail, jwt);
        if (res.type === "error") {
            alert(`Deleting litter failed: ${res.error}`);
        }
    }

    async function delImage() {
        const { res } = await deleteImage(
            form.id,
            jwt,
            API_PUPPY + "/delete/image",
        );

        if (res.type === "error") {
            alert(`Deleting puppy image failed: ${res.error}`);
        }
    }
</script>

<div class="flex flex-col">
    <h3
        class="border-t border-black p-4 text-center text-2xl font-semibold dark:border-white"
    >
        Puppies
    </h3>
    <form
        method="POST"
        id="puppyform"
        class="flex flex-col"
        enctype="multipart/form-data"
        use:enhance
    >
        <label class="hidden">
            <input type="hidden" name="id" bind:value={form.id} />
        </label>
        <label class="hidden">
            <input type="hidden" name="litter_id" bind:value={form.litter_id} />
        </label>
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Name</p>
            <input
                class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                type="text"
                name="name"
                required
                bind:value={form.name}
            />
        </label>
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Gender</p>
            <select
                class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                name="gender"
                required
                bind:value={form.gender}
            >
                <option value={Gender.MALE}>{Gender.MALE}</option>
                <option value={Gender.FEMALE}>{Gender.FEMALE}</option>
            </select>
        </label>
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Availability</p>
            <select
                class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                name="availability"
                required
                bind:value={form.availability}
            >
                <option value={Availability.AVAILABLE}>
                    {Availability.AVAILABLE}
                </option>
                <option value={Availability.UNAVAILABLE}>
                    {Availability.UNAVAILABLE}
                </option>
                <option value={Availability.CO_OWNERSHIP}>
                    {Availability.CO_OWNERSHIP}
                </option>
            </select>
        </label>
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Image</p>
            <input class="w-2/3 p-2" type="file" name="image" />
        </label>
    </form>
    <Gallery bind:images={form.image} admin on:image={delImage} />
    <div class="flex flex-row flex-wrap justify-center gap-4 p-4">
        <button
            class="h-12 w-48 rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
            on:click|preventDefault={() => (litterm = true)}
        >
            {#if litterp === undefined}
                <p>Loading litters...</p>
            {:else}
                {#await litterp}
                    <p>Loading litters...</p>
                {:then litters}
                    {#if litters === undefined}
                        <p>Server error</p>
                    {:else}
                        <p>Select litter</p>
                        <Modal
                            items={litters}
                            bind:open={litterm}
                            on:select={selectLitter}
                            on:delete={delLitter}
                        />
                    {/if}
                {:catch}
                    <p>Server error</p>
                {/await}
            {/if}
        </button>
        <button
            class="h-12 w-48 rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
            on:click|preventDefault={() => (puppym = true)}
        >
            {#if namep === undefined}
                <p>Loading puppies...</p>
            {:else}
                {#await namep}
                    <p>Loading puppies...</p>
                {:then names}
                    {#if names === undefined}
                        <p>No litter selected</p>
                    {:else}
                        <p>Select puppy</p>
                        <Modal
                            items={names}
                            bind:open={puppym}
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
            form="puppyform"
            formaction="?/puppyCreate"
        >
            Create
        </button>
        <button
            class="h-12 w-48 rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
            form="puppyform"
            formaction="?/puppyUpdate"
        >
            Update
        </button>
    </div>
</div>
