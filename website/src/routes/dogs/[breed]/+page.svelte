<script lang="ts">
    import Dogs from "$lib/components/dog/Dogs.svelte";
    import type { PageData } from "./$types";
    import { page } from "$app/stores";
    import { format } from "svelte-i18n";
    import { Breed, Gender } from "$lib/enums";

    export let data: PageData;

    $: [male, female, retired] = data.data;
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
            <Dogs dogs={male.data} {breed} />
        </div>
        <div class="flex flex-col gap-4 p-1 md:w-1/3">
            <h3 class="p-2 text-center text-2xl font-semibold">
                {$format(`dog.${Gender.FEMALE}`)}
            </h3>
            <Dogs dogs={female.data} {breed} />
        </div>
        <div class="flex flex-col gap-4 p-1 md:w-1/3">
            <h3 class="p-2 text-center text-2xl font-semibold">
                {$format("dog.retired")}
            </h3>
            <Dogs dogs={retired.data} {breed} />
        </div>
    </div>
</div>
