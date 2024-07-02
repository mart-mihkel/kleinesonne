<script lang="ts">
    import { Gallery } from "$lib/components";
    import type { PageData } from "./$types";

    export let data: PageData;
</script>

<div class="flex flex-col md:px-44 lg:px-96">
    <h2 class="p-4 text-center text-4xl">News</h2>
    {#each data.news as { title, date, text, images }}
        <div
            class="flex flex-col items-center border-t border-t-black pb-2 dark:border-t-white"
        >
            <h3 class="pt-2 text-center text-2xl">{title}</h3>
            <p class="pb-2">{date.toDateString()}</p>
            <p class="text-justify">{text}</p>
            {#if images.length > 0}
                <Gallery {images} />
            {/if}
        </div>
    {/each}
</div>
