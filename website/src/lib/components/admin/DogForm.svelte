<script lang="ts">
    import { enhance } from "$app/forms";
    import { Modal } from "$lib/components/admin";
    import { Gallery } from "$lib/components";
    import { Breed, Gender, type Name } from "$lib/types";
    import { formDate } from "$lib";
    import {
        API_DOG,
        deleteDog,
        deleteImage,
        fetchDog,
        fetchDogNames,
        resdata,
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
    let promise: Promise<Name[] | undefined> | undefined = undefined;

    onMount(() => (promise = loadNames()));

    async function loadNames(): Promise<Name[] | undefined> {
        const res = await fetchDogNames();
        const data = resdata(res);
        return data.data;
    }

    function reset() {
        form = { ...INITIAL_DATA };
    }

    async function select(e: CustomEvent<number>) {
        const res = await fetchDog(e.detail);
        const data = resdata(res);

        if (data.error) {
            alert(`Getting dog failed: ${data.error}`);
            return;
        }

        const dog = data.data!;
        form = {
            ...dog,
            thumbnail: dog.thumbnail ? [dog.thumbnail] : [],
            dob: formDate(dog.dob),
            titles: dog.titles.join(","),
            health: dog.health.join(","),
        };
    }

    async function del(e: CustomEvent<number>) {
        const { res } = await deleteDog(e.detail, jwt);
        if (res.type === "error") {
            alert(`Deleting dog failed: ${res.error}`);
        }
    }

    async function delThumbnail() {
        const { res } = await deleteImage(
            form.id,
            jwt,
            API_DOG + "/delete/thumbnail",
        );

        if (res.type === "error") {
            alert(`Deleting thumbnail failed: ${res.error}`);
        }
    }

    async function delImage(e: CustomEvent<string>) {
        const { res } = await deleteImage(
            form.id,
            jwt,
            API_DOG + "/delete/image",
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
        <div class="flex flex-row flex-wrap justify-center gap-4 p-4">
            {#if promise !== undefined}
                <button
                    class="h-12 w-48 rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                    on:click|preventDefault={() => (modal = true)}
                >
                    {#await promise}
                        <p>Loading names...</p>
                    {:then names}
                        {#if names === undefined}
                            <p>Server error</p>
                        {:else}
                            <p>Select</p>
                            <Modal
                                bind:open={modal}
                                items={names}
                                on:select={select}
                                on:delete={del}
                            />
                        {/if}
                    {:catch}
                        <p>Server error</p>
                    {/await}
                </button>
            {/if}
            <button
                class="h-12 w-48 rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                on:click|preventDefault={reset}
            >
                Reset form
            </button>
            <button
                class="h-12 w-48 rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                formaction="?/dogCreate"
            >
                Create
            </button>
            <button
                class="h-12 w-48 rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                formaction="?/dogUpdate"
            >
                Update
            </button>
        </div>
    </form>
</div>
