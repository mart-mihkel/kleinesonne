<script lang="ts">
    import type { Image } from "$lib/types";
    import { Next, Prev, Close } from "$lib/components/svg";

    export let images: Image[];

    let count = images.length;
    let show = false;
    let idx = 0;
    $: active = images[idx];

    function open(i: number) {
        show = true;
        idx = i;
    }

    function close(e?: KeyboardEvent) {
        if (e && e.key !== "Escape") {
            return;
        }

        show = false;
    }
</script>

<svelte:window on:keydown={(e) => close(e)} />

{#if show}
    <div
        class="fixed left-0 top-0 size-full overflow-hidden bg-black bg-opacity-50"
    >
        <div class="flex size-full flex-row items-center justify-center">
            <button on:click={() => (idx = (idx - 1 + count) % count)}>
                <Prev />
            </button>
            <div class="flex flex-col p-4">
                <img src={active.src} alt={active.alt} loading="lazy" />
                <div class="flex flex-row justify-center gap-3 p-4">
                    {#each images as _img, i}
                        <button
                            on:click={() => open(i)}
                            class="size-3 rounded-full transition-all duration-300"
                            class:bg-white={i === idx}
                            class:bg-gray-300={i !== idx}
                            class:opacity-50={i !== idx}
                        ></button>
                    {/each}
                </div>
            </div>
            <button on:click={() => (idx = (idx + 1) % count)}>
                <Next />
            </button>
            <button class="absolute right-2 top-2" on:click={() => close()}>
                <Close />
            </button>
        </div>
    </div>
{/if}
<div class="flex w-full flex-row flex-wrap">
    {#each images as { src, alt }, i}
        <button
            class="aspect-square w-1/2 p-1 outline-none lg:w-1/3"
            on:click={() => open(i)}
        >
            <img class="size-full object-cover" {src} {alt} loading="lazy" />
        </button>
    {/each}
</div>
