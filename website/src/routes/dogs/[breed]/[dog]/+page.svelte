<script lang="ts">
    import type { PageData } from "./$types";
    import { Dog, FamilyTree } from "$lib/components";

    export let data: PageData;

    $: dog = data.dog.data;
    $: family = data.family.data;
</script>

<div class="flex flex-col md:px-[5%] lg:px-[25%]">
    <Dog {dog} />
    <FamilyTree {family} />
</div>
