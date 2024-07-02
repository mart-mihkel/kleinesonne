<script lang="ts">
    import { Dropdown } from "$lib/components";
    import { Sun, Moon, Facebook } from "$lib/components/svg";
    import { Breed } from "$lib/types";
    import { toggle, isDark } from "$lib/dark-mode";
    import { onMount } from "svelte";

    const puppies = [
        {
            href: "/puppies/" + Breed.AUSTRALIAN,
            text: "Australian Shepherd puppies",
            title: "Australian Shepherd puppies",
        },
        {
            href: "/puppies/" + Breed.ENTLEBUCHER,
            text: "Entlebucher Mountain Dog puppies",
            title: "Entlebucher Mountain Dog puppies",
        },
        {
            href: "/puppies/" + Breed.BERNESE,
            text: "Bernese Mountain Dog puppies",
            title: "Bernese Mountain Dog puppies",
        },
    ];

    const dogs = [
        {
            href: "/dogs/" + Breed.AUSTRALIAN,
            text: "Australian Shepherds",
            title: "Australian Shepherds",
        },
        {
            href: "/dogs/" + Breed.ENTLEBUCHER,
            text: "Entlebucher Mountain Dogs",
            title: "Entlebucher Mountain Dogs",
        },
        {
            href: "/dogs/" + Breed.BERNESE,
            text: "Bernese Mountain Dogs",
            title: "Bernese Mountain Dogs",
        },
    ];

    let dark = false;
    onMount(() => (dark = isDark()));
</script>

<header
    class="flex w-full flex-row items-center gap-2 p-2 md:px-44 lg:px-96 dark:bg-black dark:text-white"
>
    <a
        class="text-2xl transition-colors duration-300 hover:text-gray-500"
        href="/"
    >
        Kleine Sonne
    </a>
    <span class="flex-grow"></span>
    <button on:click={() => (dark = toggle())}>
        {#if dark}
            <Moon />
        {:else}
            <Sun />
        {/if}
    </button>
    <a
        href="https://www.facebook.com/p/Kleine-Sonne-100064871220699/"
        target="_blank"
    >
        <Facebook />
    </a>
</header>
<nav
    class="flex flex-row gap-8 border-b border-b-black md:px-44 lg:px-96 dark:border-b-white dark:bg-black dark:text-white"
>
    <a class="transition-colors duration-300 hover:text-gray-500" href="/news">
        News
    </a>
    <Dropdown href="/puppies" items={puppies}>Puppies</Dropdown>
    <Dropdown items={dogs}>Dogs</Dropdown>
    <a
        class="transition-colors duration-300 hover:text-gray-500"
        href="/litters"
    >
        Litters
    </a>
    <a
        class="transition-colors duration-300 hover:text-gray-500"
        href="/contact"
    >
        Contact
    </a>
</nav>
