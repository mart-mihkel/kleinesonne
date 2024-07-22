<script lang="ts">
    import LitterDisplay from "$lib/components/litter/LitterDisplay.svelte";
    import Empty from "$lib/components/notice/Empty.svelte";
    import Error from "$lib/components/notice/Error.svelte";
    import type { PageData } from "./$types";
    import { format } from "svelte-i18n";

    export let data: PageData;

    let litters = data.data;
</script>

<div class="md:px-[5%] lg:px-[25%]">
    <h2 class="p-4 text-center text-4xl font-bold">
        {$format("puppies.available")}
    </h2>
    {#if litters === undefined}
        <Error message={$format("puppies.error")} />
    {:else if litters.length === 0}
        <Empty text={$format("puppies.empty")} />
    {:else}
        {#each litters as litter}
            <div class="border-t border-black pb-10 dark:border-white">
                <LitterDisplay {litter} available={true} />
            </div>
        {/each}
        <p>
            {$format("puppies.info")}
        </p>
    {/if}
</div>
