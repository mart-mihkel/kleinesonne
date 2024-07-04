<script lang="ts">
    import type { PageData } from "./$types";
    import { Litter as LitterDisplay, Loading, Error } from "$lib/components";
    import { Availability, Breed, Gender, type Litter } from "$lib/types";
    import { slide } from "svelte/transition";

    const LITTERS: Litter[] = [
        {
            name: "Entlebuch Marakratid",
            parents: { src: "/parents.webp", alt: "parents" },
            breed: Breed.ENTLEBUCH,
            images: [],
            puppies: [
                {
                    image: { src: "/usin.webp", alt: "usin" },
                    name: "Kleine Sonne Golf in Leuk",
                    gender: Gender.MALE,
                    availability: Availability.AVAILABLE,
                },
                {
                    image: { src: "/usin.webp", alt: "usin" },
                    name: "Kleine Sonne Golf in Davos",
                    gender: Gender.MALE,
                    availability: Availability.AVAILABLE,
                },
                {
                    image: { src: "/usin.webp", alt: "usin" },
                    name: "Kleine Sonne Golf in Erlen",
                    gender: Gender.FEMALE,
                    availability: Availability.CO_OWNERSHIP,
                },
                {
                    image: { src: "/usin.webp", alt: "usin" },
                    name: "Kleine Sonne Golf in Freakazoid",
                    gender: Gender.FEMALE,
                    availability: Availability.UNAVAILABLE,
                },
            ],
        },
        {
            name: "Australian Mürakarud",
            parents: { src: "/parents_aasa.webp", alt: "parents aasa" },
            breed: Breed.AUSTRALIAN,
            images: [
                { src: "/rand.webp", alt: "rand" },
                { src: "/rand.webp", alt: "rand" },
                { src: "/rand.webp", alt: "rand" },
                { src: "/rand.webp", alt: "rand" },
                { src: "/rand.webp", alt: "rand" },
                { src: "/rand.webp", alt: "rand" },
                { src: "/rand.webp", alt: "rand" },
            ],
            puppies: [
                {
                    image: { src: "/rand.webp", alt: "rand" },
                    name: "Kleine Sonne Ice Golem",
                    gender: Gender.MALE,
                    availability: Availability.AVAILABLE,
                },
                {
                    image: { src: "/rand.webp", alt: "rand" },
                    name: "Kleine Sonne Green Goblin",
                    gender: Gender.FEMALE,
                    availability: Availability.CO_OWNERSHIP,
                },
                {
                    image: { src: "/rand.webp", alt: "rand" },
                    name: "Kleine Sonne All Might",
                    gender: Gender.FEMALE,
                    availability: Availability.UNAVAILABLE,
                },
            ],
        },
        {
            name: "Olematud Olendid",
            parents: { src: "", alt: "pole olemas" },
            breed: Breed.AUSTRALIAN,
            images: [],
            puppies: [
                {
                    image: { src: "", alt: "mitte midagi" },
                    name: "Kleine Sonne Not Real",
                    gender: Gender.MALE,
                    availability: Availability.CO_OWNERSHIP,
                },
            ],
        },
    ];

    export let data: PageData;

    let litter: Promise<Litter | undefined>;
    let active = "";
    let extended = true;

    async function fetchLitter(name: string) {
        const match = LITTERS.find((l) => l.name === name);
        active = name;

        litter = new Promise((resolve) => {
            setTimeout(() => resolve(match), 1000);
        });
    }

    data.names.then((ns) => (active = ns[0]));

    $: active, fetchLitter(active);
</script>

<h2 class="p-4 text-center text-4xl">Litters</h2>
<div class="flex flex-col md:flex-row md:px-[5%] lg:px-[25%]">
    <div
        class="relative flex flex-col border-t border-black md:w-1/4 dark:border-white"
    >
        {#if extended}
            <div class="flex flex-col" transition:slide>
                {#await data.names}
                    <Loading text={"Loading litter names..."} />
                {:then names}
                    <button
                        class="p-2 md:hidden"
                        on:click={() => (extended = false)}
                    >
                        <p>Close litters menu</p>
                    </button>
                    {#each names as name}
                        <button
                            class="text-nowrap p-2"
                            class:bg-black={active === name}
                            class:text-white={active === name}
                            class:dark:bg-white={active === name}
                            class:dark:text-black={active === name}
                            class:hover:bg-gray-300={active !== name}
                            class:hover:dark:bg-gray-600={active !== name}
                            on:click={() => (active = name)}
                        >
                            {name}
                        </button>
                    {/each}
                {:catch}
                    <Error
                        message="Failed to load litter names, something went wrong"
                    />
                {/await}
            </div>
        {:else}
            <button class="p-2" on:click={() => (extended = true)}>
                Open litters menu
            </button>
        {/if}
    </div>
    <div class="border-t border-black md:w-3/4 dark:border-white">
        {#await litter}
            <Loading text={"Loading litter..."} />
        {:then litter}
            {#if litter === undefined}
                <h2 class="py-4 text-center text-2xl">
                    There is no such litter
                </h2>
            {:else}
                <LitterDisplay {litter} />
            {/if}
        {:catch}
            <Error message="Failed to load litter, something went wrong" />
        {/await}
    </div>
</div>
