<script lang="ts">
    import { enhance } from "$app/forms";
    import { Modal } from "$lib/components/admin";
    import { Loading, Error } from "$lib/components";
    import { Breed, type Litter, type Name } from "$lib/types";
    import { fetchLitter, fetchLitterNames } from "$lib/api";

    let name = "";
    let breed = Breed.AUSTRALIAN;
    let parents_image = "";
    let images: string[] = [];
    let puppies = "";

    let modal = false;
    let names: Promise<Name[]> = fetchLitterNames();
    let loading: Promise<Litter>;

    function reset() {
        name = "";
        breed = Breed.AUSTRALIAN;
        parents_image = "";
        images = [];
    }

    function select(e: CustomEvent<number>) {
        loading = fetchLitter(e.detail);
        loading.then((l) => {
            if (l === undefined) {
                reset();
                return;
            }

            name = l.name;
            breed = l.breed;
            parents_image = l.parents_image;
            images = l.images;
            puppies = ""; // TODO: get
        });
    }

    function del(e: CustomEvent<number>) {
        // TODO: server side things
        console.log("delete", e.detail);
    }

    function delImg(path: string) {
        // TODO: server side things
        console.log("delete img", path);
        images.splice(images.indexOf(path), 1);
        images = images;
    }
</script>

<div class="flex flex-col">
    <h3
        class="border-t border-black p-4 text-center text-2xl font-semibold dark:border-white"
    >
        Litters
    </h3>
    {#await loading}
        <Loading text="Loading form data..." />
    {:catch}
        <Error message="Failed to load form data, something went wrong" />
    {/await}
    {#await names}
        <Loading text="Loading modal data..." />
    {:then titles}
        <Modal
            bind:open={modal}
            items={titles}
            on:select={select}
            on:delete={del}
        />
    {:catch}
        <Error message="Failed to load litters, something went wrong" />
    {/await}
    <form
        method="POST"
        class="flex flex-col"
        enctype="multipart/form-data"
        use:enhance
    >
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Litter name</p>
            <input
                class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                type="text"
                name="name"
                required
                bind:value={name}
            />
        </label>
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Puppies (csv)</p>
            <input
                class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                type="text"
                name="puppies"
                required
                bind:value={puppies}
            />
        </label>
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Breed</p>
            <select
                class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                name="breed"
                required
                bind:value={breed}
            >
                <option value={Breed.AUSTRALIAN}>{Breed.AUSTRALIAN}</option>
                <option value={Breed.ENTLEBUCH}>{Breed.ENTLEBUCH}</option>
                <option value={Breed.BERNESE}>{Breed.BERNESE}</option>
            </select>
        </label>
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Parents</p>
            <input class="w-2/3 p-2" type="file" name="parents" />
        </label>
        {#if parents_image !== ""}
            <div class="flex w-full flex-row flex-wrap gap-4">
                <div class="flex w-full flex-row items-center gap-4">
                    <img
                        class="size-full object-cover"
                        loading="lazy"
                        src={parents_image}
                        alt=""
                    />
                    <button
                        class="size-fit rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                        on:click|preventDefault={() => delImg(parents_image)}
                    >
                        Delete thumbnail
                    </button>
                </div>
            </div>
        {/if}
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Images</p>
            <input
                class="w-2/3 p-2"
                type="file"
                name="images"
                multiple={true}
            />
        </label>
        {#if images.length > 0}
            <div class="flex w-full flex-row flex-wrap gap-4">
                {#each images as src}
                    <div class="flex w-full flex-row items-center gap-4">
                        <img
                            class="size-full object-cover"
                            loading="lazy"
                            {src}
                            alt=""
                        />
                        <button
                            class="size-fit rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                            on:click|preventDefault={() => delImg(src)}
                        >
                            Delete picture
                        </button>
                    </div>
                {/each}
            </div>
        {/if}
        <div class="flex flex-row justify-center gap-4 p-4">
            <button
                class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                on:click|preventDefault={() => (modal = true)}
            >
                Select
            </button>
            <button
                class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                on:click|preventDefault={reset}
            >
                Reset
            </button>
            <span class="flex-grow"></span>
            <button
                class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                formaction="?/litterCreate"
            >
                Create
            </button>
            <button
                class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                formaction="?/litterUpdate"
            >
                Update
            </button>
        </div>
    </form>
</div>
