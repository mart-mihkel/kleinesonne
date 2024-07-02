<script lang="ts">
    import { slide } from "svelte/transition";

    interface Item {
        href: string;
        text: string;
        title: string;
    }

    export let href = "";
    export let items: Item[];

    let show = false;
</script>

<nav
    class="relative flex flex-col"
    on:mouseenter={() => (show = true)}
    on:mouseleave={() => (show = false)}
>
    <a {href} class="transition-colors duration-300 hover:text-gray-500">
        <slot />
    </a>
    {#if show}
        <div
            class="absolute top-full z-10 flex flex-col bg-white p-1 dark:bg-black dark:text-white"
            transition:slide
        >
            {#each items as { href, text }}
                <a
                    class="text-nowrap transition-colors duration-300 hover:text-gray-500"
                    {href}
                >
                    {text}
                </a>
            {/each}
        </div>
    {/if}
</nav>
