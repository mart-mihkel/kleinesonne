<script lang="ts">
    import Close from "$lib/svg/Close.svelte";
    import Next from "$lib/svg/Next.svelte";
    import Prev from "$lib/svg/Prev.svelte";
    import type { ModalDispatch } from "$lib/types";
    import { createEventDispatcher } from "svelte";

    export let images: string[];
    export let full = false;
    export let admin = false;

    let dispatch = createEventDispatcher<ModalDispatch>();
    let count = images.length;
    let show = false;
    let idx = 0;
    $: active = images[idx];

    function open(i: number) {
        show = true;
        idx = i;
    }

    function closeKey(e: KeyboardEvent) {
        if (e.key === "Escape") {
            show = false;
        }
    }

    function close() {
        show = false;
    }

    function next() {
        idx = (idx + 1) % count;
    }

    function prev() {
        idx = (idx - 1 + count) % count;
    }

    function small(path: string): string {
        const last = path.lastIndexOf("/");
        return path.slice(0, last) + "/sm-" + path.slice(last + 1);
    }

    function del() {
        if (!admin) {
            return;
        }

        dispatch("image", active);

        images.splice(idx, 1);
        images = images;

        count = count - 1;
        idx = idx % count;
    }
</script>

<svelte:window on:keydown={(e) => closeKey(e)} />

{#if show}
    <div
        class="fixed left-0 top-0 size-full overflow-hidden bg-black bg-opacity-50"
    >
        <div class="flex size-full flex-row items-center justify-center">
            <button on:click|preventDefault={prev}>
                <Prev />
            </button>
            <div class="flex flex-col items-center gap-4 p-4">
                <img src={active} alt="" loading="lazy" />
                {#if admin}
                    <button
                        class="h-12 w-48 rounded-md border-2 border-black bg-white px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                        on:click|preventDefault={del}
                    >
                        Delete
                    </button>
                {/if}
                <div class="flex flex-row justify-center gap-3">
                    {#each [...Array(images.length).keys()] as i}
                        <button
                            on:click|preventDefault={() => open(i)}
                            class="size-3 rounded-full transition-all duration-300"
                            class:bg-white={i === idx}
                            class:bg-gray-300={i !== idx}
                            class:opacity-50={i !== idx}
                        ></button>
                    {/each}
                </div>
            </div>
            <button on:click|preventDefault={next}>
                <Next />
            </button>
            <button
                class="absolute right-2 top-2"
                on:click|preventDefault={close}
            >
                <Close />
            </button>
        </div>
    </div>
{/if}
<div class="flex w-full flex-row flex-wrap">
    {#each images as src, i}
        <button
            class="aspect-square w-1/2 p-1 outline-none lg:w-1/3"
            on:click|preventDefault={() => open(i)}
        >
            <img
                class="size-full object-cover"
                src={full ? src : small(src)}
                alt=""
                loading="lazy"
            />
        </button>
    {/each}
</div>
