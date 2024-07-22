<script lang="ts">
    import { slide } from "svelte/transition";

    type Item = {
        href: string;
        text: string;
    };

    export let href = "";
    export let items: Item[];

    let show = false;
</script>

<nav
    class="relative flex flex-col"
    on:mouseenter={() => (show = true)}
    on:mouseleave={() => (show = false)}
>
    <a
        class="text-lg font-semibold transition-colors duration-300 hover:text-gray-500"
        {href}
    >
        <slot />
    </a>
    {#if show}
        <div
            class="absolute top-full z-10 flex flex-col bg-white p-2 dark:bg-black dark:text-white"
            transition:slide
        >
            {#each items as { href, text }}
                <a
                    class="text-nowrap text-lg font-semibold transition-colors duration-300 hover:text-gray-500"
                    {href}
                >
                    {text}
                </a>
            {/each}
        </div>
    {/if}
</nav>
