<script lang="ts">
    import { Gallery, Loading, Empty, Error } from "$lib/components";
    import type { NewsPiece } from "$lib/types";
    import { onMount } from "svelte";
    import type { PageData } from "./$types";

    export let data: PageData;

    let news: NewsPiece[] = [];
    let oldest = new Date();
    $: more = data.news;
    $: loaded = false;

    let observer: IntersectionObserver;
    let bottom: Element;

    $: if (bottom !== undefined) {
        observer.observe(bottom);
    }

    data.news.then(updateNews).then(() => (loaded = true));

    onMount(() => {
        const options = { threshold: 1.0 };
        observer = new IntersectionObserver(observerCallback, options);
    });

    function observerCallback(entries: IntersectionObserverEntry[]) {
        const intersecting = entries.find((e) => e.isIntersecting);
        if (intersecting) {
            more = fetchMore(oldest);
            more.then(updateNews);
        }
    }

    function updateNews(newNews: NewsPiece[]) {
        if (newNews.length === 0) {
            observer.disconnect();
            return;
        }

        news.push(...newNews);
        news = news;
        oldest = newNews[newNews.length - 1].date;
    }

    function fetchMore(from: Date, count = 5): Promise<NewsPiece[]> {
        const fetched = [
            {
                title: "Uudis",
                date: new Date(2017, 3, 1),
                images: [{ src: "/poisid.webp", alt: "poisid" }],
                text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Morbi porttitor sapien non bibendum tincidunt.",
            },
        ]
            .filter((n) => from > n.date)
            .slice(0, count);

        return new Promise((resolve) => {
            setTimeout(() => resolve(fetched), 1500);
        });
    }
</script>

<div class="flex flex-col md:px-[5%] lg:px-[25%]">
    <h2 class="p-4 text-center text-4xl">News</h2>
    {#if news.length > 0}
        {#each news as { title, date, text, images }}
            <div
                class="flex flex-col items-center border-t border-black pb-2 dark:border-white"
            >
                <h3 class="pt-2 text-center text-2xl">{title}</h3>
                <p class="pb-2">{date.toDateString()}</p>
                <p class="text-justify">{text}</p>
                {#if images.length > 0}
                    <Gallery {images} />
                {/if}
            </div>
        {/each}
        <div bind:this={bottom}></div>
    {:else if loaded}
        <Empty />
    {/if}
    {#await more}
        <Loading text="Loading news..." />
    {:catch}
        <Error message="Failed to load news, something went wrong" />
    {/await}
</div>
