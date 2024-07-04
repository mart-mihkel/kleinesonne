<script lang="ts">
    import { type Litter } from "$lib/types";
    import { Thumbnail, Gallery } from "$lib/components";

    export let litter: Litter;

    const { name, images, parents, puppies } = litter;
</script>

<div
    class="flex flex-col items-center gap-2 border-t border-t-black py-4 dark:border-t-white"
>
    <a href="/litters/{name}" class="text-2xl">
        Litter {name}
    </a>
    <div class="flex w-full flex-row flex-wrap">
        {#each puppies as { name, gender, availability, image }}
            <div class="flex w-1/2 flex-col items-center p-1 lg:w-1/3">
                <Thumbnail {name} {gender} {availability} {image} />
            </div>
        {/each}
    </div>
    {#if images.length > 0}
        <h3 class="text-center text-2xl">Gallery</h3>
        <div class="border-y border-black dark:border-white">
            <Gallery {images} />
        </div>
    {/if}
    <h3 class="text-center text-2xl">Parents</h3>
    <img src={parents.src} alt={parents.alt} loading="lazy" />
</div>
