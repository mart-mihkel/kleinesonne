<script lang="ts">
    import { enhance } from "$app/forms";
    import { Modal } from "$lib/components/admin";
    import { Loading, Error, Gallery } from "$lib/components";
    import { Breed, type Name } from "$lib/types";
    import { deleteLitter, fetchLitter, fetchLitterNames } from "$lib/api";
    import { onMount } from "svelte";

    export let jwt: string;

    const INITIAL_DATA = {
        id: -1,
        name: "",
        breed: Breed.AUSTRALIAN,
        parents_image: <string[]>[],
        images: <string[]>[],
    };

    let form = { ...INITIAL_DATA };
    let modal = false;
    let names: Promise<Name[]>;

    onMount(() => (names = fetchLitterNames()));

    function reset() {
        form = { ...INITIAL_DATA };
    }

    async function select(e: CustomEvent<number>) {
        const litter = await fetchLitter(e.detail);
        form = { ...litter, parents_image: [litter.parents_image] };
    }

    function del(e: CustomEvent<number>) {
        deleteLitter(e.detail, jwt);
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
        <Gallery bind:images={form.parents_image} />
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Images</p>
            <input
                class="w-2/3 p-2"
                type="file"
                name="images"
                multiple={true}
            />
        </label>
        <Gallery bind:images={form.images} />
        <div class="flex flex-row justify-center gap-4 p-4">
            <button
                class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                formaction="?/litterCreate"
                on:click={() => (names = fetchLitterNames())}
            >
                Create
            </button>
            <button
                class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                formaction="?/litterUpdate"
                on:click={() => (names = fetchLitterNames())}
            >
                Update
            </button>
        </div>
    </form>
    {#await names}
        <Loading text="Loading modal data..." />
    {:then titles}
        <div class="flex flex-row justify-center gap-4 p-4">
            <button
                class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                on:click={() => (modal = true)}
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
            bind:open={modal}
            items={titles}
            on:select={select}
            on:delete={del}
        />
    {:catch}
        <Error message="Failed to load litters, something went wrong" />
    {/await}
</div>
