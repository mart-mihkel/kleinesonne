<script lang="ts">
    import { enhance } from "$app/forms";
    import { Modal } from "$lib/components/admin";
    import { Availability, Gender, type Name } from "$lib/types";
    import { Loading, Error, Gallery } from "$lib/components";
    import {
        API_PUPPY,
        deleteImage,
        deleteLitter,
        deletePuppy,
        fetchLitterNames,
        fetchPuppy,
        fetchPuppyNames,
    } from "$lib/api";
    import { onMount } from "svelte";

    export let jwt: string;

    const INITIAL_DATA = {
        id: -1,
        litter_id: -1,
        name: "",
        gender: Gender.MALE,
        availability: Availability.UNAVAILABLE,
        image: <string[]>[],
    };

    let form = { ...INITIAL_DATA };
    let litterModal = false;
    let puppyModal = false;
    let litters: Promise<Name[]>;
    let names: Promise<Name[]>;

    onMount(() => {
        names = fetchPuppyNames(form.litter_id);
        litters = fetchLitterNames();
    });

    function reset() {
        form = { ...INITIAL_DATA };
    }

    async function select(e: CustomEvent<number>) {
        const puppy = await fetchPuppy(e.detail);
        form = { ...puppy, image: puppy.image ? [puppy.image] : [] };
    }

    function del(e: CustomEvent<number>) {
        deletePuppy(e.detail, jwt);
    }

    function selectLitter(e: CustomEvent<number>) {
        form.litter_id = e.detail;
        names = fetchPuppyNames(form.litter_id);
    }

    function delLitter(e: CustomEvent<number>) {
        deleteLitter(e.detail, jwt);
    }

    function delImage() {
        deleteImage(form.id, jwt, API_PUPPY + "/delete/image");
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
        <Gallery bind:images={form.image} admin on:image={delImage} />
        <div class="flex flex-row justify-center gap-4 p-4">
            <button
                class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                formaction="?/puppyCreate"
                on:click={() => (names = fetchPuppyNames(form.litter_id))}
            >
                Create
            </button>
            <button
                class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                formaction="?/puppyUpdate"
                on:click={() => (names = fetchPuppyNames(form.litter_id))}
            >
                Update
            </button>
        </div>
    </form>
    {#await litters}
        <Loading text="Loading modal data..." />
    {:then litters}
        <div class="flex flex-row justify-center gap-4 p-4">
            <button
                class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                on:click={() => (litterModal = true)}
            >
                Select Litter
            </button>
        </div>
        <Modal
            bind:open={litterModal}
            items={litters}
            on:select={selectLitter}
            on:delete={delLitter}
        />
    {:catch}
        <Error message="Failed to load names, something went wrong" />
    {/await}
    {#await names}
        <Loading text="Loading modal data..." />
    {:then names}
        <div class="flex flex-row justify-center gap-4 p-4">
            <button
                class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                on:click={() => (puppyModal = true)}
            >
                Select
            </button>
            <button
                class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                on:click={reset}
            >
                Reset
            </button>
        </div>
        <Modal
            bind:open={puppyModal}
            items={names}
            on:select={select}
            on:delete={del}
        />
    {:catch}
        <Error message="Failed to load names, something went wrong" />
    {/await}
</div>
