<script lang="ts">
    import Close from "$lib/svg/Close.svelte";
    import Menu from "$lib/svg/Menu.svelte";
    import { format } from "svelte-i18n";
    import { slide } from "svelte/transition";

    let extended = false;

    function toggle() {
        extended = !extended;
    }
</script>

<nav class="md:hidden">
    <button class="flex w-full items-center justify-center" on:click={toggle}>
        {#if extended}
            <Close />
        {:else}
            <Menu />
        {/if}
    </button>
    {#if extended}
        <div
            transition:slide
            class="flex flex-col items-center gap-2 md:hidden"
        >
            <a
                class="text-lg font-semibold transition-colors duration-300 hover:text-gray-500"
                href="/puppies"
            >
                {$format("nav.puppies")}
            </a>
            <a
                class="text-lg font-semibold transition-colors duration-300 hover:text-gray-500"
                href="/dogs"
            >
                {$format("nav.dogs")}
            </a>
            <a
                class="text-lg font-semibold transition-colors duration-300 hover:text-gray-500"
                href="/litters"
            >
                {$format("nav.litters")}
            </a>
            <a
                class="text-lg font-semibold transition-colors duration-300 hover:text-gray-500"
                href="/news"
            >
                {$format("nav.news")}
            </a>
            <a
                class="text-lg font-semibold transition-colors duration-300 hover:text-gray-500"
                href="/contact"
            >
                {$format("nav.contact")}
            </a>
        </div>
    {/if}
</nav>
