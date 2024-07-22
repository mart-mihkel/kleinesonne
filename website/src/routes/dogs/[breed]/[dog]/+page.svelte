<script lang="ts">
    import Dog from "$lib/components/dog/Dog.svelte";
    import FamilyTree from "$lib/components/dog/FamilyTree.svelte";
    import type { PageData } from "./$types";

    export let data: PageData;

    $: dog = data.dog.data;
    $: family = data.family.data;
</script>

<div class="flex flex-col md:px-[5%] lg:px-[25%]">
    <Dog {dog} />
    <FamilyTree {family} />
</div>
