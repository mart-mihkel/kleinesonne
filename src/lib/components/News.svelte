<script lang="ts">
    interface NewsPreview {
        cover: string;
        title: string;
        date: string;
    }

    const items: NewsPreview[] = [
        {
            cover: "kohver.webp",
            title: "Jubedad jüngrid käisid pikal rabaretkel ja nägid ahvenaid!",
            date: "12.04.2023",
        },
        {
            cover: "poisid.webp",
            title: "Karm koer",
            date: "19.05.2024",
        },
        {
            cover: "viisakas.webp",
            title: "Talvepäev",
            date: "03.06.2024",
        },
        {
            cover: "usin.webp",
            title: "Uus pesakond berni apli karjakoeri",
            date: "04.06.2024",
        },
    ];
</script>

<div class="flex snap-x gap-8 overflow-x-auto bg-stone-400 py-12">
    <div class="w-auto shrink-0 snap-center"></div>
    {#each items as item}
        <div
            class="group relative w-2/3 shrink-0 snap-center bg-stone-100 shadow-lg shadow-stone-900 transition-all duration-500 ease-out md:w-1/3"
        >
            <h2
                class="absolute left-1/2 top-1 z-10 w-full -translate-x-1/2 text-center text-sm font-bold text-stone-50 md:text-lg"
            >
                {item.title}
            </h2>
            <p
                class="absolute bottom-1 left-1 z-10 -translate-y-full font-semibold text-stone-50 opacity-0 transition-all duration-700 group-hover:translate-y-0 group-hover:opacity-100"
            >
                {item.date}
            </p>
            <img
                class="h-full w-full object-cover transition-all duration-500 group-hover:brightness-50"
                src={item.cover}
                alt={item.cover}
                loading="lazy"
            />
        </div>
    {/each}
    <div class="shrink-0 snap-center"></div>
</div>
