<script lang="ts">
    import DogForm from "$lib/components/forms/DogForm.svelte";
    import FamilyForm from "$lib/components/forms/FamilyForm.svelte";
    import LitterForm from "$lib/components/forms/LitterForm.svelte";
    import NewsForm from "$lib/components/forms/NewsForm.svelte";
    import PuppyForm from "$lib/components/forms/PuppyForm.svelte";
    import type { PageData } from "./$types";

    export let data: PageData;

    let { jwt } = data;
</script>

<div class="flex flex-col gap-4 py-4 md:px-[5%] lg:px-[25%]">
    <h2 class="p-4 text-center text-4xl font-bold">Admin</h2>
    {#if jwt !== undefined}
        <NewsForm {jwt} />
        <DogForm {jwt} />
        <FamilyForm {jwt} />
        <LitterForm {jwt} />
        <PuppyForm {jwt} />
    {:else}
        <h3
            class="border-t border-black p-4 text-center text-2xl font-semibold dark:border-white"
        >
            Login
        </h3>
        <form class="flex flex-col items-center" method="post" action="?/login">
            <label class="flex flex-row items-center p-2">
                <p class="w-1/3 font-semibold">Username:</p>
                <input
                    class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                    type="text"
                    name="user"
                    required
                />
            </label>
            <label class="flex flex-row items-center p-2">
                <p class="w-1/3 font-semibold">Password:</p>
                <input
                    class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                    type="password"
                    name="secret"
                    required
                />
            </label>
            <button
                class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
            >
                Log in
            </button>
        </form>
    {/if}
</div>
