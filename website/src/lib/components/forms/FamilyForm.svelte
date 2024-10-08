<script lang="ts">
    import Modal from "../Modal.svelte";
    import type { Name } from "$lib/types";
    import { enhance } from "$app/forms";
    import { onMount } from "svelte";
    import { resdata } from "$lib/api";
    import { deleteDog, fetchDog, fetchDogNames } from "$lib/api/dog";

    export let jwt: string;

    const INITIAL_DATA = {
        name: "",
        father: "",
        mother: "",
    };

    let form = { ...INITIAL_DATA };
    let modal = false;
    let promise: Promise<Name[] | undefined> | undefined = undefined;

    onMount(() => (promise = loadNames()));

    async function loadNames(): Promise<Name[] | undefined> {
        const res = await fetchDogNames();
        const data = resdata(res);
        return data.data;
    }

    function reset() {
        form = { ...INITIAL_DATA };
    }

    async function select(e: CustomEvent<number>) {
        const res = await fetchDog(e.detail);
        const data = resdata(res);

        if (data.error) {
            alert(`Getting dog failed: ${data.error}`);
            return;
        }

        const dog = data.data!;
        form = { ...dog };
    }

    async function del(e: CustomEvent<number>) {
        const { res } = await deleteDog(e.detail, jwt);
        if (res.type === "error") {
            alert(`Deleting dog failed: ${res.error}`);
        }
    }
</script>

<div class="flex flex-col">
    <h3
        class="border-t border-black p-4 text-center text-2xl font-semibold dark:border-white"
    >
        Family Tree
    </h3>
    <form
        method="POST"
        id="familyform"
        class="flex flex-col"
        enctype="multipart/form-data"
        use:enhance
    >
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Name:</p>
            <input
                class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                type="text"
                name="name"
                required
                bind:value={form.name}
            />
        </label>
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Father</p>
            <input
                class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                type="text"
                name="father"
                required
                bind:value={form.father}
            />
        </label>
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Mother</p>
            <input
                class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                type="text"
                name="mother"
                required
                bind:value={form.mother}
            />
        </label>
    </form>
    <div class="flex flex-row flex-wrap justify-center gap-4 p-4">
        <button
            class="h-12 w-48 rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
            on:click|preventDefault={() => (modal = true)}
        >
            {#if promise === undefined}
                <p>Loading names...</p>
            {:else}
                {#await promise}
                    <p>Loading names...</p>
                {:then names}
                    {#if names === undefined}
                        <p>Server error</p>
                    {:else}
                        <p>Select</p>
                        <Modal
                            bind:open={modal}
                            items={names}
                            on:select={select}
                            on:delete={del}
                        />
                    {/if}
                {:catch}
                    <p>Server error</p>
                {/await}
            {/if}
        </button>
        <button
            class="h-12 w-48 rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
            on:click|preventDefault={reset}
        >
            Reset form
        </button>
        <button
            class="h-12 w-48 rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
            form="familyform"
            formaction="?/familyUpdate"
        >
            Update
        </button>
    </div>
</div>
