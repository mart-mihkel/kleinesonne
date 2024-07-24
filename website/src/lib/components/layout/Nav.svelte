<script lang="ts">
    import { format } from "svelte-i18n";
    import { slide } from "svelte/transition";
    import { Breed } from "$lib/enums";
    import Dropdown from "./Dropdown.svelte";

    const i18opts = {
        aus: { values: { breed: $format(`nav.dog.${Breed.AUSTRALIAN}`) } },
        ent: { values: { breed: $format(`nav.dog.${Breed.ENTLEBUCH}`) } },
        ber: { values: { breed: $format(`nav.dog.${Breed.BERNESE}`) } },
    };

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
            text: $format("dog.our", i18opts.aus),
        },
        {
            href: "/dogs/" + Breed.ENTLEBUCH,
            text: $format("dog.our", i18opts.ent),
        },
        {
            href: "/dogs/" + Breed.BERNESE,
            text: $format("dog.our", i18opts.ber),
        },
    ];
</script>

<nav class="hidden md:block">
    <div
        transition:slide
        class="flex flex-row items-center gap-8 md:px-[5%] lg:px-[25%] dark:text-white"
    >
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
</nav>
