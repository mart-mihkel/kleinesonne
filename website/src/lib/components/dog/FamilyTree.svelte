<script lang="ts">
    import Error from "$lib/components/notice/Error.svelte";
    import type { Family } from "$lib/types";
    import { format } from "svelte-i18n";

    export let family: Family | undefined;
</script>

{#if family === undefined}
    <Error message={$format("dog.error.tree")} />
{:else}
    <div class="flex flex-col border-t border-black pb-4 dark:border-white">
        <h2 class="p-4 text-center text-4xl">{$format("dog.tree")}</h2>
        <table>
            <tbody>
                <tr>
                    <td
                        class="border border-black text-center font-medium dark:border-white"
                        rowspan="4"
                    >
                        {family.father?.name ?? ""}
                    </td>
                    <td
                        class="border border-black text-center font-medium dark:border-white"
                        rowspan="2"
                    >
                        {family.father?.father?.name ?? ""}
                    </td>
                    <td
                        class="border border-black text-center font-medium dark:border-white"
                    >
                        {family.father?.father?.father?.name ?? ""}
                    </td>
                </tr>
                <tr>
                    <td
                        class="border border-black text-center font-medium dark:border-white"
                    >
                        {family.father?.father?.mother?.name ?? ""}
                    </td>
                </tr>
                <tr>
                    <td
                        class="border border-black text-center font-medium dark:border-white"
                        rowspan="2"
                    >
                        {family.father?.mother?.name ?? ""}
                    </td>
                    <td
                        class="border border-black text-center font-medium dark:border-white"
                    >
                        {family.father?.mother?.father?.name ?? ""}
                    </td>
                </tr>
                <tr>
                    <td
                        class="border border-black text-center font-medium dark:border-white"
                    >
                        {family.father?.mother?.mother?.name ?? ""}
                    </td>
                </tr>
                <tr>
                    <td
                        class="border border-black text-center font-medium dark:border-white"
                        rowspan="4"
                    >
                        {family.mother?.name ?? ""}
                    </td>
                    <td
                        class="border border-black text-center font-medium dark:border-white"
                        rowspan="2"
                    >
                        {family.mother?.father?.name ?? ""}
                    </td>
                    <td
                        class="border border-black text-center font-medium dark:border-white"
                    >
                        {family.mother?.father?.father?.name ?? ""}
                    </td>
                </tr>
                <tr>
                    <td
                        class="border border-black text-center font-medium dark:border-white"
                    >
                        {family.mother?.father?.mother?.name ?? ""}
                    </td>
                </tr>
                <tr>
                    <td
                        class="border border-black text-center font-medium dark:border-white"
                        rowspan="2"
                    >
                        {family.mother?.mother?.name ?? ""}
                    </td>
                    <td
                        class="border border-black text-center font-medium dark:border-white"
                    >
                        {family.mother?.mother?.father?.name ?? ""}
                    </td>
                </tr>
                <tr>
                    <td
                        class="border border-black text-center font-medium dark:border-white"
                    >
                        {family.mother?.mother?.mother?.name ?? ""}
                    </td>
                </tr>
            </tbody>
        </table>
    </div>
{/if}
