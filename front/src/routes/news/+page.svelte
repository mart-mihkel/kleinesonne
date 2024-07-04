<script lang="ts">
    import { Gallery, Loading, Error } from "$lib/components";
    import type { PageData } from "./$types";

    export let data: PageData;

    const news = data.news;
</script>

<div class="flex flex-col md:px-[5%] lg:px-[25%]">
    <h2 class="p-4 text-center text-4xl">News</h2>
    {#await news}
        <Loading text="Loading news..." />
    {:then news}
        {#each news as { title, date, text, images }}
            <div
                class="flex flex-col items-center border-t border-black pb-2 dark:border-white"
            >
                <h3 class="pt-2 text-center text-2xl">{title}</h3>
                <p class="pb-2">{date.toDateString()}</p>
                <p class="text-justify">{text}</p>
                {#if images.length > 0}
                    <Gallery {images} />
                {/if}
            </div>
        {/each}
    {:catch}
        <Error message="Failed to load news, something went wrong" />
    {/await}
</div>
