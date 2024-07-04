<script lang="ts">
    import type { PageData } from "./$types";
    import { Litter, Loading, Error } from "$lib/components";

    export let data: PageData;
</script>

<div class="md:px-[5%] lg:px-[25%]">
    {#await data.litters}
        <Loading text={"Loading available puppies..."} />
    {:then litters}
        {#if litters.length === 0}
            <h2 class="p-4 text-center text-4xl">
                There are no puppies available right now
            </h2>
        {:else}
            <h2 class="p-4 text-center text-4xl">Available puppies</h2>
            {#each litters as litter}
                <Litter {litter} />
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
