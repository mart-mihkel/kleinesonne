<script lang="ts">
    import type { PageData } from "./$types";
    import { page } from "$app/stores";
    import { Litter, Loading, Error, Empty } from "$lib/components";
    import { format } from "svelte-i18n";

    export let data: PageData;

    $: translated = $format(`nav.dog.${$page.params.breed}`);
    $: options = { values: { breed: translated } };
</script>

<div class="md:px-[5%] lg:px-[25%]">
    <h2 class="p-4 text-center text-4xl font-bold">
        {$format("puppies.available_breed", options)}
    </h2>
    {#await data.litters}
        <Loading text={$format("puppies.loading_breed", options)} />
    {:then litters}
        {#if litters.length === 0}
            <Empty text={$format("puppies.empty_breed", options)} />
        {:else}
            {#each litters as litter}
                <div class="border-t border-black pb-10 dark:border-white">
                    <Litter {litter} />
                </div>
            {/each}
            <p>{$format("puppies.info")}</p>
        {/if}
    {:catch}
        <Error message={$format("puppies.error")} />
    {/await}
</div>
