<div
    class="flex flex-col items-center justify-center gap-16 bg-stone-600 py-12 md:flex-row md:gap-8 md:px-8"
>
    <div class="nav-container">
        <img class="nav-image" src="kohver.jpg" alt="puppies" />
        <div class="nav-button">Puppies</div>
    </div>
    <div class="nav-container">
        <img class="nav-image" src="viisakas.jpg" alt="dogs" />
        <div class="nav-button">Dogs</div>
    </div>
    <div class="nav-container">
        <img class="nav-image" src="poisid.jpg" alt="Litters" />
        <div class="nav-button">Litters</div>
    </div>
    <div class="nav-container">
        <img class="nav-image" src="usin.jpg" alt="achievements" />
        <div class="nav-button">Results</div>
    </div>
    <div class="nav-container">
        <img class="nav-image" src="granda.jpg" alt="Passed" />
        <div class="nav-button">Passed</div>
    </div>
</div>

<style lang="postcss">
    .nav-container {
        @apply shadow-lg shadow-stone-900 md:w-1/6 md:duration-500 md:ease-out md:hover:-translate-y-4 md:hover:scale-105 md:hover:shadow-none;
    }

    .nav-image {
        @apply h-80 w-auto object-cover md:h-64;
    }

    .nav-button {
        @apply flex justify-center bg-stone-200 py-2 text-2xl font-semibold;
    }
</style>
