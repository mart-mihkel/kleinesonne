<div
    class="flex flex-col items-center justify-center gap-16 bg-stone-600 py-12 md:flex-row md:gap-8 md:px-8"
>
    <div class="nav-container">
        <img class="nav-image" src="kohver.webp" alt="puppies" />
        <div class="nav-button">Puppies</div>
    </div>
    <div class="nav-container">
        <img class="nav-image" src="viisakas.webp" alt="dogs" />
        <div class="nav-button">Dogs</div>
    </div>
    <div class="nav-container">
        <img class="nav-image" src="poisid.webp" alt="Litters" />
        <div class="nav-button">Litters</div>
    </div>
    <div class="nav-container">
        <img class="nav-image" src="usin.webp" alt="achievements" />
        <div class="nav-button">Achievements</div>
    </div>
    <div class="nav-container">
        <img class="nav-image" src="granda.webp" alt="Passed" />
        <div class="nav-button">On vacation</div>
    </div>
</div>

<style lang="postcss">
    .nav-container {
        @apply shadow-lg shadow-stone-900 md:w-1/6 md:duration-500 md:ease-out md:hover:-translate-y-4 md:hover:scale-105 md:hover:shadow-none;
    }

    .nav-image {
        @apply h-80 w-full object-cover md:h-64;
    }

    .nav-button {
        @apply flex justify-center bg-stone-200 py-2 text-2xl font-semibold;
    }
</style>
