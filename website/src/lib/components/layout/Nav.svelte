<script lang="ts">
    import { format } from "svelte-i18n";
    import { slide } from "svelte/transition";
    import { Breed } from "$lib/enums";
    import Dropdown from "./Dropdown.svelte";

    let extended = true;

    $: puppies = [
        {
            href: "/puppies/" + Breed.AUSTRALIAN,
            text: $format(`nav.puppy.${Breed.AUSTRALIAN}`),
        },
        {
            href: "/puppies/" + Breed.ENTLEBUCH,
            text: $format(`nav.puppy.${Breed.ENTLEBUCH}`),
        },
        {
            href: "/puppies/" + Breed.BERNESE,
            text: $format(`nav.puppy.${Breed.BERNESE}`),
        },
    ];

    $: dogs = [
        {
            href: "/dogs/" + Breed.AUSTRALIAN,
            text: $format(`nav.dog.${Breed.AUSTRALIAN}`),
        },
        {
            href: "/dogs/" + Breed.ENTLEBUCH,
            text: $format(`nav.dog.${Breed.ENTLEBUCH}`),
        },
        {
            href: "/dogs/" + Breed.BERNESE,
            text: $format(`nav.dog.${Breed.BERNESE}`),
        },
    ];
</script>

<nav>
    {#if extended}
        <div
            transition:slide
            class="flex flex-col items-center gap-2 border-b border-black md:flex-row md:gap-8 md:px-[5%] lg:px-[25%] dark:border-white dark:bg-black dark:text-white"
        >
            <button
                class="font-medium md:hidden"
                on:click={() => (extended = false)}
            >
                <p>{$format("nav.close")}</p>
            </button>
            <Dropdown href="/puppies" items={puppies}>
                {$format("nav.puppies")}
            </Dropdown>
            <Dropdown href="/dogs" items={dogs}>
                {$format("nav.dogs")}
            </Dropdown>
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
    {:else}
        <button
            class="flex w-full flex-col items-center border-b border-black font-medium dark:border-white"
            on:click={() => (extended = true)}
        >
            {$format("nav.open")}
        </button>
    {/if}
</nav>
