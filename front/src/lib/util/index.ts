import { Breed, type LongBreed } from "$lib/types";

export function longBreed(breed: Breed): LongBreed {
	switch (breed) {
		case Breed.AUSTRALIAN: return "Australian Shepherd";
		case Breed.BERNESE: return "Bernese Mountain Dog";
		case Breed.ENTLEBUCHER: return "Entlebucher Mountain Dog";
	}
}
