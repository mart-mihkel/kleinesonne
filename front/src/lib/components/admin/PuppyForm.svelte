<script lang="ts">
    import { enhance } from "$app/forms";
    import { Modal } from "$lib/components/admin";
    import { Availability, Gender, type Puppy } from "$lib/types";
    import { Loading, Error } from "$lib/components";
    import { fetchPuppy, fetchPuppyNames } from "$lib/mock-server";

    let name = "";
    let gender = Gender.MALE;
    let availability = Availability.UNAVAILABLE;
    let image = "";

    let modal = false;
    let names: Promise<string[]> = fetchPuppyNames();
    let loading: Promise<Puppy | undefined>;

    function reset() {
        name = "";
        gender = Gender.MALE;
        availability = Availability.UNAVAILABLE;
        image = "";
    }

    function select(e: CustomEvent<string>) {
        loading = fetchPuppy(e.detail);
        loading.then((p) => {
            if (p === undefined) {
                reset();
                return;
            }

            name = p.name;
            gender = p.gender;
            availability = p.availability;
            image = p.image;
        });
    }

    function del(e: CustomEvent<string>) {
        // TODO: server side things
        console.log("delete", e.detail);
    }

    function delImg(path: string) {
        // TODO: server side things
        console.log("delete img", path);
    }
</script>

<div class="flex flex-col">
    <h3
        class="border-t border-black p-4 text-center text-2xl font-semibold dark:border-white"
    >
        Puppies
    </h3>
    {#await loading}
        <Loading text="Loading form data..." />
    {:catch}
        <Error message="Failed to load form data, something went wrong" />
    {/await}
    {#await names}
        <Loading text="Loading modal data..." />
    {:then names}
        <Modal
            bind:open={modal}
            items={names}
            on:select={select}
            on:delete={del}
        />
    {:catch}
        <Error message="Failed to load names, something went wrong" />
    {/await}
    <form
        method="POST"
        class="flex flex-col"
        enctype="multipart/form-data"
        use:enhance
    >
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Name</p>
            <input
                class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                type="text"
                name="name"
                required
                bind:value={name}
            />
        </label>
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Gender</p>
            <select
                class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                name="breed"
                required
                bind:value={gender}
            >
                <option value={Gender.MALE}>{Gender.MALE}</option>
                <option value={Gender.FEMALE}>{Gender.FEMALE}</option>
            </select>
        </label>
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Availability</p>
            <select
                class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                name="availability"
                required
                bind:value={availability}
            >
                <option value={Availability.AVAILABLE}>
                    {Availability.AVAILABLE}
                </option>
                <option value={Availability.UNAVAILABLE}>
                    {Availability.UNAVAILABLE}
                </option>
                <option value={Availability.CO_OWNERSHIP}>
                    {Availability.CO_OWNERSHIP}
                </option>
            </select>
        </label>
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Image</p>
            <input class="w-2/3 p-2" type="file" name="image" />
        </label>
        {#if image !== ""}
            <div class="flex w-full flex-row flex-wrap gap-4">
                <div class="flex w-full flex-row items-center gap-4">
                    <img
                        class="size-full object-cover"
                        loading="lazy"
                        src={image}
                        alt=""
                    />
                    <button
                        class="size-fit rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                        on:click|preventDefault={() => delImg(image)}
                    >
                        Delete picture
                    </button>
                </div>
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
                formaction="?/puppyCreate"
            >
                Create
            </button>
            <button
                class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                formaction="?/puppyUpdate"
            >
                Update
            </button>
        </div>
    </form>
</div>
