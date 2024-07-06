<script lang="ts">
    import { NewsForm, LitterForm, DogForm } from "$lib/components/admin";
</script>

<div class="flex flex-col gap-4 py-4 md:px-[5%] lg:px-[25%]">
    <h2 class="p-4 text-center text-4xl font-bold">Admin</h2>
    <NewsForm />
    <LitterForm />
    <DogForm />
</div>
