<script lang="ts">
    import type { PageData } from "./$types";
    import { Gallery, Loading, Error } from "$lib/components";
    import { format } from "svelte-i18n";

    export let data: PageData;
</script>

<div class="flex flex-col md:px-[5%] lg:px-[25%]">
    {#await data.dog}
        <Loading text={$format("dog.loading.info")} />
    {:then { name, nickname, father, mother, dob, breed, gender, health, awards, thumbnail, images }}
        <div class="flex flex-col md:flex-row">
            <div class="md:w-1/2 md:p-2">
                <img
                    class="size-full object-cover"
                    src={thumbnail.src}
                    alt={thumbnail.alt}
                    loading="lazy"
                />
            </div>
            <div class="flex flex-col md:w-1/2">
                <h2 class="p-4 text-center text-4xl font-bold">{name}</h2>
                <table class="size-full">
                    <tbody>
                        <tr>
                            <td class="font-medium">
                                {$format("dog.nickname")}:
                            </td>
                            <td class="font-medium">{nickname}</td>
                        </tr>
                        <tr class="border-t border-black dark:border-white">
                            <td class="font-medium">
                                {$format("dog.father")}:</td
                            >
                            <td class="font-medium">{father}</td>
                        </tr>
                        <tr class="border-t border-black dark:border-white">
                            <td class="font-medium">
                                {$format("dog.mother")}:
                            </td>
                            <td class="font-medium">{mother}</td>
                        </tr>
                        <tr class="border-t border-black dark:border-white">
                            <td class="font-medium">{$format("dog.dob")}:</td>
                            <td class="font-medium">{dob.toDateString()}</td>
                        </tr>
                        <tr class="border-t border-black dark:border-white">
                            <td class="font-medium">{$format("dog.breed")}:</td>
                            <td class="font-medium">
                                {$format(`breed.${breed}.one`)}
                            </td>
                        </tr>
                        <tr class="border-t border-black dark:border-white">
                            <td class="font-medium">
                                {$format("dog.gender")}:
                            </td>
                            <td class="font-medium">
                                {$format(`dog.${gender}`)}
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
        <div class="flex flex-row border-y border-black pb-4 dark:border-white">
            <div class="flex w-full flex-col">
                <h2 class="p-4 text-center text-4xl font-bold">
                    {$format("dog.health")}
                </h2>
                <ul class="list-disc pl-[25%]">
                    {#each health as h}
                        <li class="font-medium">{h}</li>
                    {/each}
                </ul>
            </div>
            <div class="flex w-full flex-col">
                <h2 class="p-4 text-center text-4xl font-bold">
                    {$format("dog.titles")}
                </h2>
                <ul class="list-disc pl-[25%]">
                    {#each awards as a}
                        <li class="font-medium">{a}</li>
                    {/each}
                </ul>
            </div>
        </div>
        <Gallery {images} />
    {:catch}
        <Error message={$format("dog.loading.info")} />
    {/await}
    <div class="flex flex-col border-t border-black pb-4 dark:border-white">
        <h2 class="p-4 text-center text-4xl">{$format("dog.tree")}</h2>
        {#await data.tree}
            <Loading text={$format("dog.loading.tree")} />
        {:then { father, mother }}
            <table>
                <tbody>
                    <tr>
                        <td
                            class="border border-black text-center font-medium dark:border-white"
                            rowspan="4"
                        >
                            {father?.name}
                        </td>
                        <td
                            class="border border-black text-center font-medium dark:border-white"
                            rowspan="2"
                        >
                            {father?.father?.name}
                        </td>
                        <td
                            class="border border-black text-center font-medium dark:border-white"
                        >
                            {father?.father?.father?.name}
                        </td>
                    </tr>
                    <tr>
                        <td
                            class="border border-black text-center font-medium dark:border-white"
                        >
                            {father?.father?.mother?.name}
                        </td>
                    </tr>
                    <tr>
                        <td
                            class="border border-black text-center font-medium dark:border-white"
                            rowspan="2"
                        >
                            {father?.mother?.name}
                        </td>
                        <td
                            class="border border-black text-center font-medium dark:border-white"
                        >
                            {father?.mother?.father?.name}
                        </td>
                    </tr>
                    <tr>
                        <td
                            class="border border-black text-center font-medium dark:border-white"
                        >
                            {father?.mother?.mother?.name}
                        </td>
                    </tr>
                    <tr>
                        <td
                            class="border border-black text-center font-medium dark:border-white"
                            rowspan="4"
                        >
                            {mother?.name}
                        </td>
                        <td
                            class="border border-black text-center font-medium dark:border-white"
                            rowspan="2"
                        >
                            {mother?.father?.name}
                        </td>
                        <td
                            class="border border-black text-center font-medium dark:border-white"
                        >
                            {mother?.father?.father?.name}
                        </td>
                    </tr>
                    <tr>
                        <td
                            class="border border-black text-center font-medium dark:border-white"
                        >
                            {mother?.father?.mother?.name}
                        </td>
                    </tr>
                    <tr>
                        <td
                            class="border border-black text-center font-medium dark:border-white"
                            rowspan="2"
                        >
                            {mother?.mother?.name}
                        </td>
                        <td
                            class="border border-black text-center font-medium dark:border-white"
                        >
                            {mother?.mother?.father?.name}
                        </td>
                    </tr>
                    <tr>
                        <td
                            class="border border-black text-center font-medium dark:border-white"
                        >
                            {mother?.mother?.mother?.name}
                        </td>
                    </tr>
                </tbody>
            </table>
        {:catch}
            <Error message={$format("dog.error.tree")} />
        {/await}
    </div>
</div>
