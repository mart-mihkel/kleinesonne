<script lang="ts">
    import { enhance } from "$app/forms";
    import { Modal } from "$lib/components/admin";
    import { Loading, Error, Gallery } from "$lib/components";
    import { Breed, Gender, type Name } from "$lib/types";
    import { formDate } from "$lib";
    import {
        API_DOG,
        deleteDog,
        deleteImage,
        fetchDog,
        fetchDogNames,
    } from "$lib/api";
    import { onMount } from "svelte";

    export let jwt: string;

    const INITIAL_DATA = {
        id: -1,
        name: "",
        nickname: "",
        father: "",
        mother: "",
        thumbnail: <string[]>[],
        dob: "",
        breed: Breed.AUSTRALIAN,
        gender: Gender.MALE,
        alive: true,
        images: <string[]>[],
        titles: "",
        health: "",
    };

    let form = { ...INITIAL_DATA };
    let modal = false;
    let names: Promise<Name[]>;

    onMount(() => (names = fetchDogNames()));

    function reset() {
        form = { ...INITIAL_DATA };
    }

    async function select(e: CustomEvent<number>) {
        const dog = await fetchDog(e.detail);
        form = {
            ...dog,
            thumbnail: dog.thumbnail ? [dog.thumbnail] : [],
            dob: formDate(dog.dob),
            titles: dog.titles.join(","),
            health: dog.health.join(","),
        };
    }

    function del(e: CustomEvent<number>) {
        deleteDog(e.detail, jwt);
    }

    function delThumbnail() {
        deleteImage(form.id, jwt, API_DOG + "/delete/thumbnail");
    }

    function delImage(e: CustomEvent<string>) {
        deleteImage(form.id, jwt, API_DOG + "/delete/image", e.detail);
    }
</script>

<div class="flex flex-col">
    <h3
        class="border-t border-black p-4 text-center text-2xl font-semibold dark:border-white"
    >
        Dogs
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
            <p class="w-1/3 font-semibold">Name:</p>
            <input
                class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                type="text"
                name="name"
                required
                bind:value={form.name}
            />
        </label>
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Nickname</p>
            <input
                class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                type="text"
                name="nickname"
                required
                bind:value={form.nickname}
            />
        </label>
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Father</p>
            <input
                class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                type="text"
                name="father"
                required
                bind:value={form.father}
            />
        </label>
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Mother</p>
            <input
                class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                type="text"
                name="mother"
                required
                bind:value={form.mother}
            />
        </label>
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Health (csv)</p>
            <input
                class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                type="text"
                name="health"
                bind:value={form.health}
            />
        </label>
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Titles (csv)</p>
            <input
                class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                type="text"
                name="titles"
                bind:value={form.titles}
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
            <p class="w-1/3 font-semibold">Alive</p>
            <select
                class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                name="alive"
                required
                bind:value={form.alive}
            >
                <option value={true}>alive</option>
                <option value={false}>not alive</option>
            </select>
        </label>
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Thumbnail</p>
            <input class="w-2/3 p-2" type="file" name="thumbnail" />
        </label>
        <Gallery bind:images={form.thumbnail} admin on:image={delThumbnail} />
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Images</p>
            <input
                class="w-2/3 p-2"
                type="file"
                name="images"
                multiple={true}
            />
        </label>
        <Gallery bind:images={form.images} admin on:image={delImage} />
        <div class="flex flex-row justify-center gap-4 p-4">
            <button
                class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                formaction="?/dogCreate"
                on:click={() => (names = fetchDogNames())}
            >
                Create
            </button>
            <button
                class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                formaction="?/dogUpdate"
                on:click={() => (names = fetchDogNames())}
            >
                Update
            </button>
        </div>
    </form>
    {#await names}
        <Loading text="Loading dog names..." />
    {:then names}
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
            items={names}
            on:select={select}
            on:delete={del}
        />
    {:catch}
        <Error message="Failed to load names, something went wrong" />
    {/await}
</div>
