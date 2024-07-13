<script lang="ts">
    import { Loading, Error, Empty } from "$lib/components";
    import type { ModalDispatch } from "$lib/types";
    import { slide } from "svelte/transition";
    import { createEventDispatcher } from "svelte";
    import { fetchLitterNames } from "$lib/api";
    import { format } from "svelte-i18n";

    let dispatch = createEventDispatcher<ModalDispatch>();
    let extended = true;
    let active = 0;
    let names = fetchLitterNames();

    names.then((ns) => {
        if (ns.length === 0) {
            return;
        }

        const id = ns[0].id;
        active = id;
        dispatch("select", id);
    });
</script>

{#if extended}
    <div class="flex flex-col" transition:slide>
        {#await names}
            <Loading text={$format("litter.loading.many")} />
        {:then names}
            <button
                class="p-2 font-medium md:hidden"
                on:click={() => (extended = false)}
            >
                <p>{$format("litter.close")}</p>
            </button>
            {#if names.length > 0}
                {#each names as { id, name }}
                    <button
                        class="text-nowrap p-2 font-semibold transition-colors duration-300 hover:bg-gray-200 dark:border-white dark:hover:bg-gray-500"
                        class:bg-gray-300={active === id}
                        class:dark:bg-gray-700={active === id}
                        on:click={() => dispatch("select", id)}
                    >
                        {name}
                    </button>
                {/each}
            {:else}
                <Empty />
            {/if}
        {:catch}
            <Error message={$format("litter.error.many")} />
        {/await}
    </div>
{:else}
    <button class="p-2 font-medium" on:click={() => (extended = true)}>
        {$format("litter.open")}
    </button>
{/if}
