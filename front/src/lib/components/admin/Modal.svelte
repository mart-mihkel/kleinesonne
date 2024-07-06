<script lang="ts">
    import { type ModalDispatch } from "$lib/types";
    import { createEventDispatcher } from "svelte";

    export let open: boolean;
    export let active: string;
    export let items: string[];

    let dispatch = createEventDispatcher<ModalDispatch>();
    let filter: string;
    $: filtered = filter
        ? items.filter((i) => i.toLowerCase().includes(filter.toLowerCase()))
        : items;

    function pick() {
        if (!active) {
            return;
        }

        open = false;
        dispatch("select", active);
    }

    function del() {
        if (!active) {
            return;
        }

        items.splice(items.indexOf(active), 1);
        items = items;
        dispatch("delete", active);
    }

    function close() {
        open = false;
    }
</script>

{#if open}
    <div
        class="fixed left-0 top-0 size-full overflow-hidden bg-black bg-opacity-50 dark:text-black"
    >
        <div class="flex size-full items-center justify-center">
            <div class="flex flex-col gap-4 rounded-lg bg-white p-4 shadow-lg">
                <div class="flex flex-row items-center gap-2">
                    <p class="font-medium">Filter:</p>
                    <input
                        class="rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                        type="text"
                        bind:value={filter}
                    />
                </div>
                <div
                    class="flex h-[300px] w-full flex-col overflow-auto border-2 border-black"
                >
                    {#each filtered as item}
                        <button
                            class="px-4 py-2 text-center font-medium"
                            class:hover:bg-gray-300={active !== item}
                            class:bg-gray-500={active === item}
                            on:click={() => (active = item)}
                        >
                            {item}
                        </button>
                    {/each}
                </div>
                <button
                    class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                    on:click={pick}
                >
                    Select
                </button>
                <button
                    class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                    on:click={del}
                >
                    Delete
                </button>
                <button
                    class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300"
                    on:click={close}
                >
                    Cancel
                </button>
            </div>
        </div>
    </div>
{/if}
