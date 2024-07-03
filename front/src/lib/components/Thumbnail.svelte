<script lang="ts">
    export let href: string | undefined = undefined;
    export let nickname: string | undefined = undefined;
    export let name: string;
    export let src: string;
    export let alt: string;
</script>

<a
    class="group relative flex flex-col items-center"
    class:cursor-default={href === undefined}
    {href}
>
    <div class="overflow-hidden">
        <img
            class="aspect-square size-full object-cover transition-transform duration-300 group-hover:scale-110"
            loading="lazy"
            {src}
            {alt}
        />
    </div>
    <div
        class="absolute bottom-0 w-full bg-white bg-opacity-75 transition-opacity duration-300 group-hover:opacity-0 dark:bg-black dark:bg-opacity-50"
    >
        {#if nickname}
            <h3 class="p-2 text-center text-2xl">{nickname}</h3>
        {/if}
        <h4 class="text-center text-lg">{name}</h4>
    </div>
</a>
