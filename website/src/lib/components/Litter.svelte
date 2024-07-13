<script lang="ts">
    import { type LitterWithPuppies } from "$lib/types";
    import { Thumbnail, Gallery } from "$lib/components";
    import { format } from "svelte-i18n";

    export let litter: LitterWithPuppies;

    const { name, images, parents_image, puppies } = litter;
</script>

<div class="flex flex-col items-center gap-2 py-4">
    <a href="/litters/{name}" class="text-2xl font-semibold">
        {$format("litter.litter", { values: { name } })}
    </a>
    <div class="flex w-full flex-row flex-wrap">
        {#each puppies as { name, gender, availability, image }}
            <div class="flex w-1/2 flex-col items-center p-1 lg:w-1/3">
                <Thumbnail {name} {gender} {availability} src={image} />
            </div>
        {/each}
    </div>
    {#if images.length > 0}
        <h3 class="text-center text-2xl font-semibold">
            {$format("litter.gallery")}
        </h3>
        <div class="border-y border-black dark:border-white">
            <Gallery {images} />
        </div>
    {/if}
    <h3 class="text-center text-2xl font-semibold">
        {$format("litter.parents")}
    </h3>
    <img src={parents_image} alt="" loading="lazy" />
</div>
