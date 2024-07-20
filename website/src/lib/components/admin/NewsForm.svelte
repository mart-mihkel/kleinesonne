<script lang="ts">
    import { enhance } from "$app/forms";
    import { onMount } from "svelte";
    import { formDate } from "$lib";
    import type { Name } from "$lib/types";
    import { Modal } from "$lib/components/admin";
    import { Gallery } from "$lib/components";
    import {
        deleteImage,
        deleteArticle,
        fetchArticle,
        fetchTitles,
        API_NEWS,
        resdata,
    } from "$lib/api";

    export let jwt: string;

    const INITIAL_DATA = {
        id: -1,
        title: "",
        date: "",
        text: "",
        images: <string[]>[],
    };

    let form = { ...INITIAL_DATA };
    let modal = false;
    let promise: Promise<Name[] | undefined> | undefined = undefined;

    onMount(() => (promise = loadTitles()));

    async function loadTitles(): Promise<Name[] | undefined> {
        const res = await fetchTitles();
        const data = resdata(res);
        return data.data;
    }

    function reset() {
        form = { ...INITIAL_DATA };
    }

    async function select(e: CustomEvent<number>) {
        const res = await fetchArticle(e.detail);
        const data = resdata(res);

        if (data.error) {
            alert(`Getting dog failed: ${data.error}`);
            return;
        }

        const article = data.data!;
        form = { ...article, date: formDate(article.date) };
    }

    async function del(e: CustomEvent<number>) {
        const { res } = await deleteArticle(e.detail, jwt);
        if (res.type === "error") {
            alert(`Deleting article failed: ${res.error}`);
        }
    }

    async function delImage(e: CustomEvent<string>) {
        const { res } = await deleteImage(
            form.id,
            jwt,
            API_NEWS + "/delete/image",
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
        News
    </h3>
    <form
        method="POST"
        id="newsform"
        class="flex flex-col"
        enctype="multipart/form-data"
        use:enhance
    >
        <label class="hidden">
            <input type="hidden" name="id" bind:value={form.id} />
        </label>
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Title:</p>
            <input
                class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                type="text"
                name="title"
                required
                bind:value={form.title}
            />
        </label>
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Date</p>
            <input
                class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                type="date"
                name="date"
                required
                bind:value={form.date}
            />
        </label>
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Text</p>
            <input
                class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                type="text"
                name="text"
                required
                bind:value={form.text}
            />
        </label>
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
                <p>Loading titles...</p>
            {:else}
                {#await promise}
                    <p>Loading titles...</p>
                {:then titles}
                    {#if titles === undefined}
                        <p>Server error</p>
                    {:else}
                        <p>Select</p>
                        <Modal
                            bind:open={modal}
                            items={titles}
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
            form="newsform"
            formaction="?/newsCreate"
        >
            Create
        </button>
        <button
            class="h-12 w-48 rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
            form="newsform"
            formaction="?/newsUpdate"
        >
            Update
        </button>
    </div>
</div>
