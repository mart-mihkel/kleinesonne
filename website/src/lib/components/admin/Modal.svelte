<script lang="ts">
    import { type ModalDispatch, type Name } from "$lib/types";
    import { createEventDispatcher } from "svelte";

    export let open: boolean;
    export let items: Name[];
    export let active: number = 0;

    let dispatch = createEventDispatcher<ModalDispatch>();
    let filter: string;

    const f = (i: Name) => i.name.toLowerCase().includes(filter.toLowerCase());
    $: filtered = filter ? items.filter(f) : items;

    function pick() {
        if (!active) {
            return;
        }

        dispatch("select", active);
    }

    function del() {
        if (!active) {
            return;
        }

        const i = items.findIndex((i) => i.id === active);

        items.splice(i, 1);
        items = items;

        dispatch("delete", active);
    }

    function close(e: KeyboardEvent) {
        if (e.key === "Escape") {
            open = false;
        }
    }
</script>

<svelte:window on:keydown={close} />

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
                    {#each filtered as { id, name }}
                        <button
                            class="px-4 py-2 text-center font-medium"
                            class:hover:bg-gray-300={active !== id}
                            class:bg-gray-500={active === id}
                            on:click|preventDefault={() => (active = id)}
                        >
                            {name}
                        </button>
                    {/each}
                </div>
                <button
                    class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                    on:click|preventDefault={pick}
                >
                    Select
                </button>
                <button
                    class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                    on:click|preventDefault={del}
                >
                    Delete
                </button>
            </div>
        </div>
    </div>
{/if}
