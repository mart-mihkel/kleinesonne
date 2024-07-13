<script lang="ts">
    import type { PageData } from "./$types";
    import {
        Litter as LitterDisplay,
        Loading,
        Error,
        Empty,
        LitterList,
    } from "$lib/components";
    import { type Litter } from "$lib/types";
    import { format } from "svelte-i18n";
    import { fetchLitter } from "$lib/api";

    export let data: PageData;

    let litter: Promise<Litter>;

    function select(id: number) {
        litter = fetchLitter(id);
    }
</script>

<h2 class="p-4 text-center text-4xl font-bold">{$format("nav.litters")}</h2>
<div class="flex flex-col md:flex-row md:px-[5%] lg:px-[25%]">
    <div
        class="relative flex flex-col border-t border-black md:w-1/4 dark:border-white"
    >
        <LitterList on:select={select} />
    </div>
    <div class="border-t border-black md:w-3/4 dark:border-white">
        {#await litter}
            <Loading text={$format("litter.loading.one")} />
        {:then litter}
            {#if litter}
                <LitterDisplay {litter} />
            {:else}
                <Empty />
            {/if}
        {:catch}
            <Error message={$format("litter.error.one")} />
        {/await}
    </div>
</div>
