<script lang="ts">
    import type { PageData } from "./$types";
    import { Litter } from "$lib/components";

    export let data: PageData;
    const litters = data.litters;
</script>

<div class="md:px-44 lg:px-64">
    {#if litters.length === 0}
        <h2 class="p-4 text-center text-4xl">
            There are no puppies available right now
        </h2>
    {:else}
        <h2 class="p-4 text-center text-4xl">Available puppies</h2>
        {#each litters as litter}
            <Litter {litter} onlyAvailable={true} />
        {/each}
    {/if}
</div>
