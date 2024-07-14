<script lang="ts">
    import type { Article } from "$lib/types";
    import { Gallery } from "$lib/components";

    export let news: Article[];
</script>

{#each news as { title, date, text, images }}
    <div
        class="flex flex-col items-center border-t border-black pb-2 dark:border-white"
    >
        <h3 class="pt-2 text-center text-2xl font-semibold">{title}</h3>
        <p class="pb-2 text-lg font-medium">
            {new Date(date * 1000).toDateString()}
        </p>
        <p class="text-justify">{text}</p>
        {#if images.length > 0}
            <Gallery {images} />
        {/if}
    </div>
{/each}
