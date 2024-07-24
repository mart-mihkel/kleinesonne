<script lang="ts">
    import { Availability, Gender } from "$lib/enums";
    import { format } from "svelte-i18n";

    export let href: string | undefined = undefined;
    export let availability: Availability | undefined = undefined;
    export let gender: Gender | undefined = undefined;
    export let nickname: string | undefined = undefined;
    export let name: string;
    export let src: string | undefined;

    $: options = { values: { gender: $format(`dog.${gender}`) } };
</script>

<a
    class="relative flex w-full flex-col items-center shadow dark:shadow-gray-700"
    class:cursor-default={href === undefined}
    {href}
>
    <img
        class="aspect-square size-full object-cover"
        loading="lazy"
        src={src ?? "/default.webp"}
        alt="Dog thumbnail"
    />
    <div
        class="flex w-full flex-col gap-2 bg-gray-200 px-2 pb-2 dark:bg-gray-700"
    >
        {#if nickname}
            <h3 class="pt-2 text-center text-2xl font-bold">{nickname}</h3>
        {/if}
        <h4
            class="text-center text-lg font-semibold"
            class:pt-2={nickname === undefined}
        >
            {name}
        </h4>
        {#if gender !== undefined}
            <p class="flex items-center justify-center text-center font-medium">
                {#if availability === undefined || availability === Availability.UNAVAILABLE}
                    {$format(`dog.${gender}`)}
                {:else if availability === Availability.AVAILABLE}
                    {$format("puppies.available_gender", options)}
                {:else if availability === Availability.CO_OWNERSHIP}
                    {$format("puppies.available_gender_co", options)}
                {/if}
            </p>
        {/if}
    </div>
</a>
