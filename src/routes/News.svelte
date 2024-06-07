<script lang="ts">
    import { scale } from "svelte/transition";

    interface Item {
        id: number;
        src: string;
        alt: string;
        title: string;
        date: string;
        text: string;
    }

    let items = [
        {
            id: 0,
            src: "kohver.jpg",
            alt: "pic_1",
            title: "Jubedad jüngrid käisid pikal rabaretkel ja nägid ahvenaid!",
            date: "12.04.2023",
            text: "Nad on vangistatud. Nad on vangistatud. Nad on vangistatud. Nad on vangistatud. Nad on vangistatud. Nad on vangistatud. Nad on vangistatud. Nad on vangistatud. Nad on vangistatud. Nad on vangistatud. Nad on vangistatud. Nad on vangistatud. Nad on vangistatud. Nad on vangistatud. Nad on vangistatud. Nad on vangistatud. Nad on vangistatud. Nad on vangistatud.",
        },
        {
            id: 1,
            src: "dark_header.jpg",
            alt: "pic_2",
            title: "Karm koer",
            date: "19.05.2024",
            text: "Ta on viisakas. Ta on viisakas. Ta on viisakas. Ta on viisakas. Ta on viisakas. Ta on viisakas. Ta on viisakas. Ta on viisakas. Ta on viisakas. Ta on viisakas. Ta on viisakas. Ta on viisakas. Ta on viisakas. Ta on viisakas. Ta on viisakas. Ta on viisakas. Ta on viisakas. Ta on viisakas. Ta on viisakas. Ta on viisakas. Ta on viisakas. Ta on viisakas. Ta on viisakas. Ta on viisakas. Ta on viisakas. ",
        },
        {
            id: 2,
            src: "",
            alt: "pic_3",
            title: "title_3",
            date: "03.06.2024",
            text: "text_3",
        },
        {
            id: 3,
            src: "",
            alt: "pic_4",
            title: "title_4",
            date: "04.06.2024",
            text: "text_4",
        },
    ];
</script>

<div class="flex justify-center relative">
    {#each items as item, i}
        <button
            class="flex absolute justify-center bg-sky-50 items-center transition-all rounded-r w-1/3"
            style:transform="translateX({(i / items.length) * 100}%)"
            class:blur={i !== 0}
            class:z-10={i === 0}
            on:click={() => (items = [...items.slice(1), items[0]])}
        >
            <div class="flex-none relative w-1/3">
                <img
                    class="size-full absolute object-cover"
                    src={item.src}
                    alt={item.alt}
                />
            </div>
            <div class="flex flex-col gap-4 w-2/3">
                <h2 class="text-2xl font-semibold">{item.title}</h2>
                <h4 class="text-lg font-semibold">{item.date}</h4>
                <p class="text-sm">{item.text}</p>
            </div>
        </button>
    {/each}
</div>
