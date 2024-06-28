<script lang="ts">
    import { slide } from "svelte/transition";

    interface Item {
        href: string;
        text: string;
        title: string;
    }

    export let items: Item[];

    let show = false;
</script>

<div
    role="navigation"
    class="relative flex flex-col dark:bg-black"
    on:mouseenter={() => (show = true)}
    on:mouseleave={() => (show = false)}
>
    <p
        class="transition-colors duration-300 hover:text-slate-500 dark:text-white"
    >
        <slot />
    </p>
    {#if show}
        <div
            class="absolute top-full z-10 flex flex-col bg-white p-1 dark:bg-black"
            transition:slide
        >
            {#each items as { href, text, title }}
                <a
                    class="text-nowrap transition-colors duration-300 hover:text-slate-500 dark:text-white"
                    {href}
                    {title}
                >
                    {text}
                </a>
            {/each}
        </div>
    {/if}
</div>
