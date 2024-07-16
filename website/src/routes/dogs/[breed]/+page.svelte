<script lang="ts">
    import { Dogs } from "$lib/components";
    import { page } from "$app/stores";
    import { format } from "svelte-i18n";
    import type { PageData } from "./$types";
    import { Breed, Gender } from "$lib/types";

    export let data: PageData;

    $: breed = $page.params.breed as Breed;
    $: translated = $format(`nav.dog.${breed}`);
    $: options = { values: { breed: translated } };
</script>

<div class="flex flex-col items-center gap-4 md:px-[5%] lg:px-[25%]">
    <h2 class="p-4 text-center text-4xl font-bold">
        {$format("dog.our", options)}
    </h2>
    <div class="flex w-full flex-col pb-4 md:flex-row">
        <div class="flex flex-col gap-4 p-1 md:w-1/3">
            <h3 class="p-2 text-center text-2xl font-semibold">
                {$format(`dog.${Gender.MALE}`)}
            </h3>
            <Dogs promise={data.male} {breed} />
        </div>
        <div class="flex flex-col gap-4 p-1 md:w-1/3">
            <h3 class="p-2 text-center text-2xl font-semibold">
                {$format(`dog.${Gender.FEMALE}`)}
            </h3>
            <Dogs promise={data.female} {breed} />
        </div>
        <div class="flex flex-col gap-4 p-1 md:w-1/3">
            <h3 class="p-2 text-center text-2xl font-semibold">
                {$format("dog.retired")}
            </h3>
            <Dogs promise={data.retired} {breed} />
        </div>
    </div>
</div>
