<script lang="ts">
    import type { Breed } from "$lib/types";
    import type { PageData } from "./$types";
    import { page } from "$app/stores";
    import { longBreed } from "$lib/util";
    import { Litter } from "$lib/components";

    export let data: PageData;

    // eslint-disable-next-line svelte/valid-compile
    $: breed = $page.params.breed as Breed;

    $: litters = data.litters.filter((l) => l.breed === breed);
</script>

<div class="md:px-44 lg:px-64 dark:bg-black dark:text-white">
    {#if litters.length === 0}
        <h2 class="p-4 text-center text-4xl">
            There are no {longBreed(breed)} puppies available right now
        </h2>
    {:else}
        <h2 class="p-4 text-center text-4xl">
            Available {longBreed(breed)} puppies
        </h2>
        {#each litters as litter}
            <Litter {litter} />
        {/each}
    {/if}
</div>
