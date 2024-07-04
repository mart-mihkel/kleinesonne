<script lang="ts">
    import { Availability, Gender, type Image } from "$lib/types";

    export let href: string | undefined = undefined;
    export let availability: Availability | undefined = undefined;
    export let gender: Gender | undefined = undefined;
    export let nickname: string | undefined = undefined;
    export let name: string;
    export let image: Image;

    const { src, alt } = image;
    const gstr = gender === Gender.MALE ? "Male" : "Female";
</script>

<a
    class="group relative flex flex-col items-center shadow-md dark:shadow-white"
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
        class="flex w-full flex-col gap-2 bg-gray-50 p-2 dark:bg-white dark:text-black"
    >
        {#if nickname}
            <h3 class="p-2 text-center text-2xl">{nickname}</h3>
        {/if}
        <h4 class="text-center text-lg">{name}</h4>
        {#if gender !== undefined}
            <p class="flex h-12 items-center justify-center text-center">
                {#if availability === undefined || availability === Availability.UNAVAILABLE}
                    {gstr}
                {:else if availability === Availability.AVAILABLE}
                    Available {gstr.toLowerCase()}
                {:else if availability === Availability.CO_OWNERSHIP}
                    {gstr} available for co-ownership
                {/if}
            </p>
        {/if}
    </div>
</a>
