<script lang="ts">
    import type { ModalDispatch, Name } from "$lib/types";
    import { createEventDispatcher } from "svelte";
    import { slide } from "svelte/transition";
    import { format } from "svelte-i18n";
    import Empty from "../notice/Empty.svelte";
    import Error from "../notice/Error.svelte";
    import Close from "$lib/svg/Close.svelte";
    import Menu from "$lib/svg/Menu.svelte";

    export let names: Name[] | undefined;

    let dispatch = createEventDispatcher<ModalDispatch>();
    let extended = true;
    let active = 0;

    function select(id: number) {
        active = id;
        dispatch("select", id);
    }

    function toggle() {
        extended = !extended;
    }
</script>

<div class="flex flex-col" transition:slide>
    {#if names === undefined}
        <Error message={$format("litter.names.error")} />
    {:else if names.length === 0}
        <Empty text={$format("litter.names.empty")} />
    {:else}
        <button
            class="flex w-full items-center justify-center md:hidden"
            on:click={toggle}
        >
            {#if extended}
                <Close />
            {:else}
                <Menu />
            {/if}
        </button>
        {#if extended}
            {#each names as { id, name }}
                <button
                    class="text-nowrap p-2 font-semibold transition-colors duration-300 hover:bg-gray-200 dark:border-white dark:hover:bg-gray-500"
                    class:bg-gray-300={active === id}
                    class:dark:bg-gray-700={active === id}
                    on:click={() => select(id)}
                >
                    {name}
                </button>
            {/each}
        {/if}
    {/if}
</div>
