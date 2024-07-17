<script lang="ts">
    import { Loading, Empty, Error, News } from "$lib/components";
    import type { ApiResponse, Article } from "$lib/types";
    import type { PageData } from "./$types";
    import { format } from "svelte-i18n";
    import { onMount } from "svelte";
    import { fetchNews, resdata } from "$lib/api";

    export let data: PageData;

    // TODO: resdata error handling
    let old = data.data!;
    let oldest =
        old.length > 0
            ? old[old.length - 1].date
            : Math.floor(new Date().getTime() / 1000);

    let promise: Promise<ApiResponse<Article[]>> | undefined = undefined;
    let bottom: Element;
    let end = false;

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

        promise = fetchNews(oldest, 5);

        const res = await promise;
        const data = resdata(res);

        update(data.data!);
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
