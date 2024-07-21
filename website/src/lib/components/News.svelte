<script lang="ts">
    import { marked } from "marked";
    import type { Article } from "$lib/types";
    import { Gallery } from "$lib/components";

    export let news: Article[];
</script>

{#each news as { title, date, text, images }}
    <article
        class="flex flex-col items-center border-t border-black pb-2 dark:border-white"
    >
        <h3 class="pt-2 text-center text-4xl font-bold">{title}</h3>
        <p class="pb-2 text-lg font-semibold">
            {new Date(date).toDateString()}
        </p>
        <div class="w-full text-start">{@html marked(text)}</div>
        <Gallery {images} />
    </article>
{/each}
