<script lang="ts">
    import Error from "$lib/components/notice/Error.svelte";
    import { page } from "$app/stores";
</script>

<div
    class="flex flex-col items-center justify-center gap-4 py-4 md:px-[5%] lg:px-[25%]"
>
    {#if $page.error}
        <Error message={`Error ${$page.status}: ${$page.error.message}`} />
    {/if}
</div>
