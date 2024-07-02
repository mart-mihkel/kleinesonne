<script lang="ts">
    import { Dropdown } from "$lib/components";
    import { Sun, Moon, Facebook } from "$lib/components/svg";
    import { Breed } from "$lib/types";
    import { toggle, isDark } from "$lib/dark-mode";
    import { onMount } from "svelte";

    const puppies = [
        {
            href: "/puppies/" + Breed.AUSTRALIAN,
            text: "Australian shepherd puppies",
            title: "Australian shepherd puppies",
        },
        {
            href: "/puppies/" + Breed.BERNESE,
            text: "Bernese mountain dog puppies",
            title: "Bernese mountain dog puppies",
        },
        {
            href: "/puppies/" + Breed.ENTLEBUCHER,
            text: "Entlebucher mountain dog puppies",
            title: "Entlebucher mountain dog puppies",
        },
    ];

    const dogs = [
        {
            href: "/dogs/" + Breed.AUSTRALIAN,
            text: "Australian shepherds",
            title: "Australian shepherds",
        },
        {
            href: "/dogs/" + Breed.BERNESE,
            text: "Bernese mountain dogs",
            title: "Bernese mountain dogs",
        },
        {
            href: "/dogs/" + Breed.ENTLEBUCHER,
            text: "Entlebucher mountain dogs",
            title: "Entlebucher mountain dogs",
        },
    ];

    let dark = false;
    onMount(() => (dark = isDark()));
</script>

<header
    class="flex w-full flex-row items-center gap-2 p-2 md:px-44 lg:px-64 dark:bg-black dark:text-white"
>
    <a
        class="text-2xl transition-colors duration-300 hover:text-slate-500"
        href="/"
        title="Kleine Sonne"
    >
        Kleine Sonne
    </a>
    <div class="flex-grow"></div>
    <button on:click={() => (dark = toggle())}>
        {#if dark}
            <Moon />
        {:else}
            <Sun />
        {/if}
    </button>
    <a
        href="https://www.facebook.com/p/Kleine-Sonne-100064871220699/"
        title="Kleinesonne Facebook"
        target="_blank"
    >
        <Facebook />
    </a>
</header>
<nav
    class="flex flex-row gap-8 border-b border-b-black md:px-44 lg:px-64 dark:border-b-white dark:bg-black dark:text-white"
>
    <Dropdown items={puppies}>Puppies</Dropdown>
    <Dropdown items={dogs}>Dogs</Dropdown>
    <Dropdown items={puppies}>Litters</Dropdown>
    <a
        class="transition-colors duration-300 hover:text-slate-500"
        href="/contact"
        title="Contact page"
    >
        Contact
    </a>
</nav>
