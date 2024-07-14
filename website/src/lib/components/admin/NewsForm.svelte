<script lang="ts">
    import { enhance } from "$app/forms";
    import type { Name } from "$lib/types";
    import { Modal } from "$lib/components/admin";
    import { Loading, Error, Gallery } from "$lib/components";
    import { deleteArticle, fetchArticle, fetchTitles } from "$lib/api";
    import { onMount } from "svelte";
    import { formDate } from "$lib";

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
    let titles: Promise<Name[]>;

    onMount(() => (titles = fetchTitles()));

    function reset() {
        form = { ...INITIAL_DATA };
    }

    async function select(e: CustomEvent<number>) {
        const article = await fetchArticle(e.detail);
        form = { ...article, date: formDate(article.date) };
    }

    function del(e: CustomEvent<number>) {
        deleteArticle(e.detail, jwt);
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
        <Gallery bind:images={form.images} />
        <div class="flex flex-row justify-center gap-4 p-4">
            <button
                class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                formaction="?/newsCreate"
                on:click={() => (titles = fetchTitles())}
            >
                Create
            </button>
            <button
                class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                formaction="?/newsUpdate"
                on:click={() => (titles = fetchTitles())}
            >
                Update
            </button>
        </div>
    </form>
    {#await titles}
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
        <Error message="Failed to load titles, something went wrong" />
    {/await}
</div>
