<script lang="ts">
    import { onMount } from "svelte";

    const items = [
        { src: "rand.webp", alt: "p1", title: "t1" },
        { src: "viisakas.webp", alt: "p2", title: "t2" },
        { src: "usin.webp", alt: "p3", title: "t3" },
        { src: "usin.webp", alt: "p3", title: "t3" },
        { src: "usin.webp", alt: "p3", title: "t3" },
        { src: "usin.webp", alt: "p3", title: "t3" },
    ];
    let active = 0;

    onMount(() => {
        const interval = setInterval(
            () => (active = (active + 1) % items.length),
            3000,
        );

        return () => clearInterval(interval);
    });
</script>

<div class="flex overflow-x-visible">
    {#each items as { src, alt, title }}
        <div>
            <img {src} {alt} />
        </div>
        <h2>{title}</h2>
    {/each}
</div>
