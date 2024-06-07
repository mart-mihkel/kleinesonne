<script lang="ts">
    interface Item {
        id: number;
        src: string;
        alt: string;
        title: string;
        date: string;
        text: string;
    }

    let items: Item[] = [
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

<div class="relative flex justify-center">
    {#each items as item, i}
        <button
            class="absolute flex w-1/3 items-center justify-center rounded-r bg-sky-50 transition-all"
            style:transform="translateX({(i / items.length) * 100}%)"
            class:blur={i !== 0}
            class:z-10={i === 0}
            on:click={() => (items = [...items.slice(1), items[0]])}
        >
            <div class="relative w-1/3 flex-none">
                <img
                    class="absolute size-full object-cover"
                    src={item.src}
                    alt={item.alt}
                />
            </div>
            <div class="flex w-2/3 flex-col gap-4">
                <h2 class="text-2xl font-semibold">{item.title}</h2>
                <h4 class="text-lg font-semibold">{item.date}</h4>
                <p class="text-sm">{item.text}</p>
            </div>
        </button>
    {/each}
</div>
