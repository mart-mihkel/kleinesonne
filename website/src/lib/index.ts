import { Breed, type LongBreed } from "$lib/types";

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

export function formDate(secs: number): string {
    const date = new Date(secs * 1000);
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
    return Math.floor(new Date(yyyy, mm - 1, dd).getTime() / 1000);
}

export function smallImage(path: string): string {
    const last = path.lastIndexOf("/");
    return path.slice(0, last) + "/sm-" + path.slice(last + 1);
}
