<script lang="ts">
    import { page } from "$app/stores";
    import { ShortBreed } from "$lib/types";
    import { longBreed } from "$lib/util";

    // eslint-disable-next-line svelte/valid-compile
    $: breed = $page.params.breed as ShortBreed;
</script>

<div class="md:px-44 lg:px-64 dark:bg-black dark:text-white">
    <h2>There are no {longBreed(breed)} puppies available right now</h2>
</div>
