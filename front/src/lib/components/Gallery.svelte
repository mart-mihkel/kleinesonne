<script lang="ts">
    import type { Image } from "$lib/types";

    export let images: Image[];

    let count = images.length;
    let show = false;
    let i = 0;
    $: active = images[i];

    function open(j: number) {
        show = true;
        i = j;
    }
</script>

{#if show}
    <div
        class="fixed left-0 top-0 size-full overflow-hidden bg-black bg-opacity-50"
    >
        <div class="flex size-full flex-row items-center justify-center gap-4">
            <button on:click={() => (i = (i - 1 + count) % count)}>prev</button>
            <img src={active.src} alt={active.alt} />
            <button on:click={() => (i = (i + 1) % count)}>next</button>
            <button
                class="absolute right-2 top-2"
                on:click={() => (show = false)}
            >
                close
            </button>
        </div>
    </div>
{:else}
    {#each images as { src, alt }, j}
        <button on:click={() => open(j)}>
            <img {src} {alt} loading="lazy" />
        </button>
    {/each}
{/if}
