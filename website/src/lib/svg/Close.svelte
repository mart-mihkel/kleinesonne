<script lang="ts">
    export let gallery = false;
</script>

<svg
    class="size-6 transition-colors duration-300 hover:fill-gray-300 group-hover:fill-gray-300"
    class:fill-white={gallery}
    class:fill-black={!gallery}
    class:dark:fill-white={!gallery}
    xmlns="http://www.w3.org/2000/svg"
    viewBox="0 -960 960 960"
>
    <path
        d="m256-200-56-56 224-224-224-224 56-56 224 224 224-224 56 56-224 224 224 224-56 56-224-224-224 224Z"
    />
</svg>
