<script lang="ts">
    import { slide } from "svelte/transition";

    interface Item {
        href: string;
        text: string;
        title: string;
    }

    export let href = "";
    export let title = "";
    export let items: Item[];

    let show = false;
</script>

<nav
    class="relative flex flex-col"
    on:mouseenter={() => (show = true)}
    on:mouseleave={() => (show = false)}
>
    <a
        {href}
        {title}
        class="transition-colors duration-300 hover:text-slate-500"
    >
        <slot />
    </a>
    {#if show}
        <div
            class="absolute top-full z-10 flex flex-col bg-white p-1 dark:bg-black dark:text-white"
            transition:slide
        >
            {#each items as { href, text, title }}
                <a
                    class="text-nowrap transition-colors duration-300 hover:text-slate-500"
                    {href}
                    {title}
                >
                    {text}
                </a>
            {/each}
        </div>
    {/if}
</nav>
