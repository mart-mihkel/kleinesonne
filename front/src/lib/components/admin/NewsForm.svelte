<script lang="ts">
    import { enhance } from "$app/forms";
    import { fetchTitles, fetchNewsPiece } from "$lib/mock-server";
    import { dateInput } from "$lib/util";
    import type { NewsPiece } from "$lib/types";
    import { Modal } from "$lib/components/admin";
    import { Loading, Error } from "$lib/components";

    let title = "";
    let date = dateInput(new Date());
    let text = "";
    let images: string[] = [];

    let modal = false;
    let titles: Promise<string[]> = fetchTitles();
    let loading: Promise<NewsPiece | undefined>;

    function reset() {
        title = "";
        text = "";
        images = [];
        date = dateInput(new Date());
    }

    function select(e: CustomEvent<string>) {
        loading = fetchNewsPiece(e.detail);
        loading.then((n) => {
            if (n === undefined) {
                reset();
                return;
            }

            title = n.title;
            text = n.text;
            images = n.images;
            date = dateInput(n.date);
        });
    }

    function del(e: CustomEvent<string>) {
        // TODO: server side things
        console.log("delete", e.detail);
    }

    function delImg(path: string) {
        // TODO: server side things
        console.log("delete img", path);
        images.splice(images.indexOf(path), 1);
        images = images;
    }
</script>

<div class="flex flex-col">
    <h3
        class="border-t border-black p-4 text-center text-2xl font-semibold dark:border-white"
    >
        News
    </h3>
    {#await loading}
        <Loading text="Loading form data..." />
    {:catch}
        <Error message="Failed to load form data, something went wrong" />
    {/await}
    {#await titles}
        <Loading text="Loading modal data..." />
    {:then titles}
        <Modal
            bind:open={modal}
            items={titles}
            on:select={select}
            on:delete={del}
        />
    {:catch}
        <Error message="Failed to load titles, something went wrong" />
    {/await}
    <form
        method="POST"
        class="flex flex-col"
        enctype="multipart/form-data"
        use:enhance
    >
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Title:</p>
            <input
                class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                type="text"
                name="title"
                required
                bind:value={title}
            />
        </label>
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Date</p>
            <input
                class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                type="date"
                name="date"
                required
                bind:value={date}
            />
        </label>
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Text</p>
            <input
                class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                type="text"
                name="text"
                required
                bind:value={text}
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
        {#if images.length > 0}
            <div class="flex w-full flex-row flex-wrap gap-4">
                {#each images as src}
                    <div class="flex w-full flex-row items-center gap-4">
                        <img
                            class="size-full object-cover"
                            loading="lazy"
                            {src}
                            alt=""
                        />
                        <button
                            class="size-fit rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                            on:click|preventDefault={() => delImg(src)}
                        >
                            Delete picture
                        </button>
                    </div>
                {/each}
            </div>
        {/if}
        <div class="flex flex-row justify-center gap-4 p-4">
            <button
                class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                on:click|preventDefault={() => (modal = true)}
            >
                Select
            </button>
            <button
                class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                on:click|preventDefault={reset}
            >
                Reset
            </button>
            <span class="flex-grow"></span>
            <button
                class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                formaction="?/newsCreate"
            >
                Create
            </button>
            <button
                class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                formaction="?/newsUpdate"
            >
                Update
            </button>
        </div>
    </form>
</div>
