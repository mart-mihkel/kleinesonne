<script lang="ts">
    import type { Breed } from "$lib/types";
    import type { PageData } from "./$types";
    import { page } from "$app/stores";
    import { longBreed } from "$lib/util";
    import { Litter, Loading, Error, Empty } from "$lib/components";

    export let data: PageData;

    $: breed = $page.params.breed as Breed;
</script>

<div class="md:px-[5%] lg:px-[25%]">
    {#await data.litters}
        <Loading text={`Loading available ${longBreed(breed)} puppies...`} />
    {:then litters}
        {#if litters.length === 0}
            <h2 class="p-4 text-center text-4xl">
                There are no {longBreed(breed)} puppies available right now
            </h2>
            <Empty />
        {:else}
            <h2 class="p-4 text-center text-4xl">
                Available {longBreed(breed)} puppies
            </h2>
            {#each litters as litter}
                <div class="border-t border-black pb-10 dark:border-white">
                    <Litter {litter} />
                </div>
            {/each}
            <p>
                The puppies will leave for their new homes healthy, having been
                checked by a veterinarian, vaccinated and protected against
                parasites. The puppies are chipped, FCI registered and have an
                euro passport.
            </p>
        {/if}
    {:catch}
        <Error message="Failed to load puppies, something went wrong" />
    {/await}
</div>
