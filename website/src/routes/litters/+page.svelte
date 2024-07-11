<script lang="ts">
    import type { PageData } from "./$types";
    import {
        Litter as LitterDisplay,
        Loading,
        Error,
        Empty,
    } from "$lib/components";
    import { type Litter } from "$lib/types";
    import { slide } from "svelte/transition";
    import { format } from "svelte-i18n";
    import { fetchLitter } from "$lib/api";

    export let data: PageData;

    let promise: Promise<Litter | undefined>;
    let active = "";
    let extended = true;

    data.names.then((ns) => loadLitter(ns[0].id));

    function loadLitter(id: number) {
        active = id;
        promise = fetchLitter(id);
    }
</script>

<h2 class="p-4 text-center text-4xl font-bold">{$format("nav.litters")}</h2>
<div class="flex flex-col md:flex-row md:px-[5%] lg:px-[25%]">
    <div
        class="relative flex flex-col border-t border-black md:w-1/4 dark:border-white"
    >
        {#if extended}
            <div class="flex flex-col" transition:slide>
                {#await data.names}
                    <Loading text={$format("litter.loading.many")} />
                {:then names}
                    <button
                        class="p-2 font-medium md:hidden"
                        on:click={() => (extended = false)}
                    >
                        <p>{$format("litter.close")}</p>
                    </button>
                    {#each names as name}
                        <button
                            class="text-nowrap p-2 font-semibold transition-colors duration-300 hover:bg-gray-200 dark:border-white dark:hover:bg-gray-500"
                            class:bg-gray-300={active === name}
                            class:dark:bg-gray-700={active === name}
                            on:click={() => loadLitter(name)}
                        >
                            {name}
                        </button>
                    {/each}
                {:catch}
                    <Error message={$format("litter.error.many")} />
                {/await}
            </div>
        {:else}
            <button class="p-2 font-medium" on:click={() => (extended = true)}>
                {$format("litter.open")}
            </button>
        {/if}
    </div>
    <div class="border-t border-black md:w-3/4 dark:border-white">
        {#await promise}
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
