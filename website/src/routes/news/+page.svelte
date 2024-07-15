<script lang="ts">
    import { Loading, Empty, Error, News } from "$lib/components";
    import type { Article } from "$lib/types";
    import type { PageData } from "./$types";
    import { format } from "svelte-i18n";
    import { onMount } from "svelte";
    import { fetchNews } from "$lib/api";

    export let data: PageData;

    let promise = data.news;
    let old: Article[] = [];
    let oldest = Math.floor(new Date().getTime() / 1000);
    let end = false;
    let bottom: Element;

    promise.then(update);

    onMount(() => {
        const opts = { threshold: 1.0 };
        const observer = new IntersectionObserver(onIntersect, opts);

        observer.observe(bottom);

        return () => observer.disconnect();
    });

    async function onIntersect(entries: IntersectionObserverEntry[]) {
        const intersecting = entries.find((e) => e.isIntersecting);
        if (!intersecting) {
            return;
        }

        update(await fetchNews(oldest, 5));
    }

    function update(news: Article[]) {
        if (news.length === 0) {
            end = true;
            return;
        }

        old = [...old, ...news];
        oldest = news[news.length - 1].date;
    }
</script>

<div class="flex flex-col md:px-[5%] lg:px-[25%]">
    <h2 class="p-4 text-center text-4xl font-bold">{$format("nav.news")}</h2>
    <News bind:news={old} />
    {#await promise}
        <Loading text={$format("news.loading")} />
    {:catch}
        <Error message={$format("news.error")} />
    {/await}
    {#if end}
        <Empty text={$format("news.empty")} />
    {/if}
    <div bind:this={bottom}></div>
</div>
