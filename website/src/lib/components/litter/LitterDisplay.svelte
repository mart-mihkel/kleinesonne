<script lang="ts">
    import Gallery from "../Gallery.svelte";
    import Thumbnail from "../Thumbnail.svelte";
    import Loading from "../notice/Loading.svelte";
    import Empty from "../notice/Empty.svelte";
    import Error from "../notice/Error.svelte";
    import type { Litter, Puppy } from "$lib/types";
    import { Availability } from "$lib/enums";
    import { format } from "svelte-i18n";
    import { onMount } from "svelte";
    import { resdata } from "$lib/api";
    import { fetchPuppies } from "$lib/api/puppy";

    export let litter: Litter | undefined;
    export let available = false;

    let promise: Promise<Puppy[] | undefined> | undefined = undefined;

    onMount(() => (promise = litter ? loadPuppies(litter.id) : undefined));

    async function loadPuppies(
        litter_id: number,
    ): Promise<Puppy[] | undefined> {
        const res = await fetchPuppies(litter_id);
        const data = resdata(res);
        return data.data;
    }

    function filtered(puppies: Puppy[]): Puppy[] {
        return available
            ? puppies.filter((p) => p.availability !== Availability.UNAVAILABLE)
            : puppies;
    }
</script>

<div class="flex flex-col items-center gap-2 py-4">
    {#if litter === undefined || promise === undefined}
        <Error message={$format("litter.display.error")} />
    {:else}
        <h2 class="text-2xl font-semibold">
            {$format("litter.litter", { values: { name: litter.name } })}
        </h2>
        <p class="pb-2 text-lg font-medium">
            {new Date(litter.dob).toDateString()}
        </p>
        {#await promise}
            <Loading text={$format("litter.display.loading")} />
        {:then puppies}
            {#if puppies === undefined}
                <Error message={$format("litter.display.error")} />
            {:else if puppies.length === 0}
                <Empty text={$format("litter.display.empty")} />
            {:else}
                <div class="flex w-full flex-row flex-wrap">
                    {#each filtered(puppies) as { name, gender, availability, image }}
                        <div
                            class="flex w-1/2 flex-col items-center p-1 lg:w-1/3"
                        >
                            <Thumbnail
                                {name}
                                {gender}
                                {availability}
                                src={image}
                            />
                        </div>
                    {/each}
                </div>
            {/if}
        {:catch}
            <Error message={$format("litter.display.error")} />
        {/await}
        {#if litter.images.length > 0}
            <h3 class="text-center text-2xl font-semibold">
                {$format("litter.gallery")}
            </h3>
            <div class="border-y border-black dark:border-white">
                <Gallery images={litter.images} />
            </div>
        {/if}
        {#if litter.parents_image}
            <h3 class="text-center text-2xl font-semibold">
                {$format("litter.parents")}
            </h3>
            <img src={litter.parents_image} alt="" loading="lazy" />
        {/if}
    {/if}
</div>
