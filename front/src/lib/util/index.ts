import { Breed, ShortBreed } from "$lib/types";

export function shortBreed(breed: string): string {
	return breed.split(" ")[0].toLowerCase();
}

export function longBreed(breed: ShortBreed): Breed {
	switch (breed) {
		case ShortBreed.AUSTRALIAN: return Breed.AUSTRALIAN;
		case ShortBreed.BERNESE: return Breed.BERNESE;
		case ShortBreed.ENTLEBUCHER: return Breed.ENTLEBUCHER;
	}
}
