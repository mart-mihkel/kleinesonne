import type { LongBreed } from "$lib/types";
import { Breed } from "$lib/enums";

export function longBreed(breed: Breed): LongBreed {
    switch (breed) {
        case Breed.AUSTRALIAN:
            return "Australian Shepherd";
        case Breed.BERNESE:
            return "Bernese Mountain Dog";
        case Breed.ENTLEBUCH:
            return "Entlebuch Cattle Dog";
    }
}

export function formDate(millis: number): string {
    const date = new Date(millis);
    const m = date.getMonth() + 1;
    const d = date.getDate();

    return (
        date.getFullYear() +
        "-" +
        (m < 10 ? "0" : "") +
        m +
        "-" +
        (d < 10 ? "0" : "") +
        d
    );
}

export function parseDate(date: string): number {
    const [yyyy, mm, dd] = date.split("-").map(Number);
    return new Date(yyyy, mm - 1, dd).getTime();
}
