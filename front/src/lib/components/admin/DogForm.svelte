<script lang="ts">
    import { enhance } from "$app/forms";
    import { fetchDog, fetchDogNames } from "$lib/mock-server";
    import { dateInput } from "$lib/util";
    import { Modal } from "$lib/components/admin";
    import { Loading, Error } from "$lib/components";
    import { Breed, Gender, type Dog } from "$lib/types";

    let name = "";
    let nickname = "";
    let father = "";
    let mother = "";
    let breed = "";
    let gender = "";
    let titles = "";
    let health = "";
    let alive = true;
    let dob = dateInput(new Date());
    let images: string[] = [];
    let thumbnail = "";

    let modal = false;
    let names: Promise<string[]> = fetchDogNames();
    let loading: Promise<Dog | undefined>;

    function reset() {
        name = "";
        nickname = "";
        father = "";
        mother = "";
        breed = "";
        gender = "";
        titles = "";
        health = "";
        alive = true;
        dob = dateInput(new Date());
        images = [];
        thumbnail = "";
    }

    function select(e: CustomEvent<string>) {
        loading = fetchDog(e.detail);
        loading.then((d) => {
            if (d === undefined) {
                reset();
                return;
            }

            name = d.name;
            nickname = d.nickname;
            father = d.father;
            mother = d.mother;
            breed = d.breed;
            gender = d.gender;
            alive = d.alive;
            titles = d.titles.join(",");
            health = d.health.join(",");
            dob = dateInput(d.dob);
            images = d.images;
            thumbnail = d.thumbnail;
        });
    }

    function del(e: CustomEvent<string>) {
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
        Dogs
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
            <p class="w-1/3 font-semibold">Name:</p>
            <input
                class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                type="text"
                name="name"
                required
                bind:value={name}
            />
        </label>
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Nickname</p>
            <input
                class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                type="text"
                name="nickname"
                required
                bind:value={nickname}
            />
        </label>
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Father</p>
            <input
                class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                type="text"
                name="father"
                required
                bind:value={father}
            />
        </label>
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Mother</p>
            <input
                class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                type="text"
                name="mother"
                required
                bind:value={mother}
            />
        </label>
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Health (csv)</p>
            <input
                class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                type="text"
                name="health"
                required
                bind:value={health}
            />
        </label>
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Titles (csv)</p>
            <input
                class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                type="text"
                name="titles"
                bind:value={titles}
            />
        </label>
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Date of birth</p>
            <input
                class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                type="date"
                name="dob"
                required
                bind:value={dob}
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
            <p class="w-1/3 font-semibold">Gender</p>
            <select
                class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                name="greed"
                required
                bind:value={gender}
            >
                <option value={Gender.MALE}>{Gender.MALE}</option>
                <option value={Gender.FEMALE}>{Gender.FEMALE}</option>
            </select>
        </label>
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Alive</p>
            <select
                class="w-2/3 rounded border-2 border-gray-500 bg-white p-2 focus:border-black focus:bg-gray-200 focus:outline-none dark:text-black"
                name="alive"
                required
                bind:value={alive}
            >
                <option value={true}>alive</option>
                <option value={false}>not alive</option>
            </select>
        </label>
        <label class="flex flex-row items-center p-2">
            <p class="w-1/3 font-semibold">Thumbnail</p>
            <input class="w-2/3 p-2" type="file" name="thumbnail" />
        </label>
        {#if thumbnail !== ""}
            <div class="flex w-full flex-row flex-wrap gap-4">
                <div class="flex w-full flex-row items-center gap-4">
                    <img
                        class="size-full object-cover"
                        loading="lazy"
                        src={thumbnail}
                        alt=""
                    />
                    <button
                        class="size-fit rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                        on:click|preventDefault={() => delImg(thumbnail)}
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
                formaction="?/dogCreate"
            >
                Create
            </button>
            <button
                class="rounded-md border-2 border-black px-4 py-2 text-center font-bold transition-colors duration-300 ease-out hover:bg-gray-300 dark:border-white dark:hover:bg-gray-500"
                formaction="?/dogUpdate"
            >
                Update
            </button>
        </div>
    </form>
</div>
