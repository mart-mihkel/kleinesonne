<script lang="ts">
    import { Gallery, Error } from "$lib/components";
    import type { Dog } from "$lib/types";
    import { format } from "svelte-i18n";
    import def from "$lib/assets/default.avif";

    export let dog: Dog | undefined;
</script>

{#if dog === undefined}
    <Error message={$format("dog.error.info")} />
{:else}
    <div class="flex flex-col md:flex-row">
        <div class="md:w-1/2 md:p-2">
            <img
                class="size-full object-cover"
                src={dog.thumbnail ?? def}
                alt="Dog thumbnail"
                loading="lazy"
            />
        </div>
        <div class="flex flex-col md:w-1/2">
            <h2 class="p-4 text-center text-4xl font-bold">{dog.name}</h2>
            <table class="size-full">
                <tbody>
                    <tr>
                        <td class="font-medium">
                            {$format("dog.nickname")}:
                        </td>
                        <td class="font-medium">{dog.nickname}</td>
                    </tr>
                    <tr class="border-t border-black dark:border-white">
                        <td class="font-medium"> {$format("dog.father")}:</td>
                        <td class="font-medium">{dog.father}</td>
                    </tr>
                    <tr class="border-t border-black dark:border-white">
                        <td class="font-medium">
                            {$format("dog.mother")}:
                        </td>
                        <td class="font-medium">{dog.mother}</td>
                    </tr>
                    <tr class="border-t border-black dark:border-white">
                        <td class="font-medium">{$format("dog.dob")}:</td>
                        <td class="font-medium">
                            {new Date(dog.dob * 1000).toDateString()}
                        </td>
                    </tr>
                    <tr class="border-t border-black dark:border-white">
                        <td class="font-medium">{$format("dog.breed")}:</td>
                        <td class="font-medium">
                            {$format(`nav.dog.${dog.breed}`)}
                        </td>
                    </tr>
                    <tr class="border-t border-black dark:border-white">
                        <td class="font-medium">
                            {$format("dog.gender")}:
                        </td>
                        <td class="font-medium">
                            {$format(`dog.${dog.gender}`)}
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
                {#each dog.health as h}
                    <li class="font-medium">{h}</li>
                {/each}
            </ul>
        </div>
        <div class="flex w-full flex-col">
            <h2 class="p-4 text-center text-4xl font-bold">
                {$format("dog.titles")}
            </h2>
            <ul class="list-disc pl-[25%]">
                {#each dog.titles as t}
                    <li class="font-medium">{t}</li>
                {/each}
            </ul>
        </div>
    </div>
    <Gallery images={dog.images} />
{/if}
