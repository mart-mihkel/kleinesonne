<script lang="ts">
    import type { PageData } from "./$types";
    import { Gallery, Loading, Error } from "$lib/components";
    import { longBreed } from "$lib/util";
    import { Gender } from "$lib/types";

    export let data: PageData;
</script>

<div class="flex flex-col md:px-[5%] lg:px-[25%]">
    {#await data.dog}
        <Loading text={"Loading dog info..."} />
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
                <h2 class="p-4 text-center text-4xl">{name}</h2>
                <table class="size-full">
                    <tbody>
                        <tr>
                            <td>Nickname:</td>
                            <td>{nickname}</td>
                        </tr>
                        <tr class="border-t border-black dark:border-white">
                            <td>Father:</td>
                            <td>{father}</td>
                        </tr>
                        <tr class="border-t border-black dark:border-white">
                            <td>Mother:</td>
                            <td>{mother}</td>
                        </tr>
                        <tr class="border-t border-black dark:border-white">
                            <td>Date of birth:</td>
                            <td>{dob.toDateString()}</td>
                        </tr>
                        <tr class="border-t border-black dark:border-white">
                            <td>Breed:</td>
                            <td>{longBreed(breed)}</td>
                        </tr>
                        <tr class="border-t border-black dark:border-white">
                            <td>Gender:</td>
                            <td>{gender === Gender.MALE ? "Male" : "Female"}</td
                            >
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
        <div class="flex flex-row border-y border-black pb-4 dark:border-white">
            <div class="flex w-full flex-col">
                <h2 class="p-4 text-center text-4xl">Health</h2>
                <ul class="list-disc pl-[25%]">
                    {#each health as h}
                        <li>{h}</li>
                    {/each}
                </ul>
            </div>
            <div class="flex w-full flex-col">
                <h2 class="p-4 text-center text-4xl">Awards</h2>
                <ul class="list-disc pl-[25%]">
                    {#each awards as a}
                        <li>{a}</li>
                    {/each}
                </ul>
            </div>
        </div>
        <Gallery {images} />
    {:catch}
        <Error message="Failed to dog info, something went wrong" />
    {/await}
    <div class="flex flex-col border-t border-black pb-4 dark:border-white">
        <h2 class="p-4 text-center text-4xl">Familiy Tree</h2>
        {#await data.tree}
            <Loading text={"Loading family tree..."} />
        {:then { father, mother }}
            <table>
                <tbody>
                    <tr>
                        <td
                            class="border border-black text-center dark:border-white"
                            rowspan="4"
                        >
                            {father?.name}
                        </td>
                        <td
                            class="border border-black text-center dark:border-white"
                            rowspan="2"
                        >
                            {father?.father?.name}
                        </td>
                        <td
                            class="border border-black text-center dark:border-white"
                        >
                            {father?.father?.father?.name}
                        </td>
                    </tr>
                    <tr>
                        <td
                            class="border border-black text-center dark:border-white"
                        >
                            {father?.father?.mother?.name}
                        </td>
                    </tr>
                    <tr>
                        <td
                            class="border border-black text-center dark:border-white"
                            rowspan="2"
                        >
                            {father?.mother?.name}
                        </td>
                        <td
                            class="border border-black text-center dark:border-white"
                        >
                            {father?.mother?.father?.name}
                        </td>
                    </tr>
                    <tr>
                        <td
                            class="border border-black text-center dark:border-white"
                        >
                            {father?.mother?.mother?.name}
                        </td>
                    </tr>
                    <tr>
                        <td
                            class="border border-black text-center dark:border-white"
                            rowspan="4"
                        >
                            {mother?.name}
                        </td>
                        <td
                            class="border border-black text-center dark:border-white"
                            rowspan="2"
                        >
                            {mother?.father?.name}
                        </td>
                        <td
                            class="border border-black text-center dark:border-white"
                        >
                            {mother?.father?.father?.name}
                        </td>
                    </tr>
                    <tr>
                        <td
                            class="border border-black text-center dark:border-white"
                        >
                            {mother?.father?.mother?.name}
                        </td>
                    </tr>
                    <tr>
                        <td
                            class="border border-black text-center dark:border-white"
                            rowspan="2"
                        >
                            {mother?.mother?.name}
                        </td>
                        <td
                            class="border border-black text-center dark:border-white"
                        >
                            {mother?.mother?.father?.name}
                        </td>
                    </tr>
                    <tr>
                        <td
                            class="border border-black text-center dark:border-white"
                        >
                            {mother?.mother?.mother?.name}
                        </td>
                    </tr>
                </tbody>
            </table>
        {:catch}
            <Error message="Failed to load family tree, something went wrong" />
        {/await}
    </div>
</div>
