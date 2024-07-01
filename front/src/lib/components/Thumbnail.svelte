<script lang="ts">
    export let href: string;
    export let nickname: string;
    export let name: string;
    export let src: string;
    export let alt: string;
</script>

<a class="group relative flex flex-col items-center" {href} title={nickname}>
    <div class="w-10/12 overflow-hidden">
        <img
            class="size-full object-cover transition-transform duration-300 group-hover:scale-110"
            loading="lazy"
            {src}
            {alt}
        />
    </div>
    <div
        class="absolute bottom-0 w-10/12 bg-white bg-opacity-50 transition-opacity duration-300 group-hover:opacity-0"
    >
        <h3 class="p-2 text-center text-2xl">{nickname}</h3>
        <h4 class="text-center text-lg">{name}</h4>
    </div>
</a>
