<script lang="ts">
    import { Availability, type Litter, type Puppy } from "$lib/types";
    import { Thumbnail, Gallery, Loading, Error, Empty } from "$lib/components";
    import { format } from "svelte-i18n";
    import { fetchPuppies } from "$lib/api";

    export let litter: Litter;
    export let available = false;

    const { id, name, images, parents_image } = litter;
    let puppiesPromise: Promise<Puppy[]> = fetchPuppies(id);

    function filtered(puppies: Puppy[]): Puppy[] {
        return available
            ? puppies.filter((p) => p.availability !== Availability.UNAVAILABLE)
            : puppies;
    }
</script>

<div class="flex flex-col items-center gap-2 py-4">
    <a href="/litters/{name}" class="text-2xl font-semibold">
        {$format("litter.litter", { values: { name } })}
    </a>
    {#await puppiesPromise}
        <Loading text={$format("litter.display.loading")} />
    {:then puppies}
        {#if puppies.length > 0}
            <div class="flex w-full flex-row flex-wrap">
                {#each filtered(puppies) as { name, gender, availability, image }}
                    <div class="flex w-1/2 flex-col items-center p-1 lg:w-1/3">
                        <Thumbnail {name} {gender} {availability} src={image} />
                    </div>
                {/each}
            </div>
        {:else}
            <Empty text={$format("litter.display.empty")} />
        {/if}
    {:catch}
        <Error message={$format("litter.display.error")} />
    {/await}
    {#if images.length > 0}
        <h3 class="text-center text-2xl font-semibold">
            {$format("litter.gallery")}
        </h3>
        <div class="border-y border-black dark:border-white">
            <Gallery {images} />
        </div>
    {/if}
    {#if parents_image}
        <h3 class="text-center text-2xl font-semibold">
            {$format("litter.parents")}
        </h3>
        <img src={parents_image} alt="" loading="lazy" />
    {/if}
</div>
