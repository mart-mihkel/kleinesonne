<script lang="ts">
    import { type Litter, type Puppy } from "$lib/types";
    import { Thumbnail, Gallery, Loading, Error } from "$lib/components";
    import { format } from "svelte-i18n";
    import { fetchPuppies } from "$lib/api";

    export let litter: Litter;

    const { id, name, images, parents_image } = litter;
    let puppiesPromise: Promise<Puppy[]> = fetchPuppies(id);
</script>

<div class="flex flex-col items-center gap-2 py-4">
    <a href="/litters/{name}" class="text-2xl font-semibold">
        {$format("litter.litter", { values: { name } })}
    </a>
    {#await puppiesPromise}
        <Loading text="Loading puppies..." />
    {:then puppies}
        <div class="flex w-full flex-row flex-wrap">
            {#each puppies as { name, gender, availability, image }}
                <div class="flex w-1/2 flex-col items-center p-1 lg:w-1/3">
                    <Thumbnail {name} {gender} {availability} src={image} />
                </div>
            {/each}
        </div>
    {:catch}
        <Error message="Failed to load puppies, something went wrong" />
    {/await}
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
