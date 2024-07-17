<script lang="ts">
    import { Availability, type Litter, type Puppy } from "$lib/types";
    import { Thumbnail, Gallery, Loading, Error, Empty } from "$lib/components";
    import { format } from "svelte-i18n";
    import { fetchPuppies, resdata } from "$lib/api";

    export let litter: Litter | undefined;
    export let available = false;

    let promise: Promise<Puppy[] | undefined> | undefined = litter
        ? new Promise(async (resolve) => {
              const res = await fetchPuppies(litter.id);
              const data = resdata(res);
              resolve(data.data);
          })
        : undefined;

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
        <a href="/litters/{litter.name}" class="text-2xl font-semibold">
            {$format("litter.litter", { values: { name: litter.name } })}
        </a>
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
