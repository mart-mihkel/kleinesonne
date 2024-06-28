export enum Gender {
	MALE,
	FEMALE,
}

export enum Breed {
	AUSTRALIAN = "Australian Shepherd",
	BERNESE = "Bernese Mountain Dog",
	ENTLEBUCHER = "Entlebucher Cattle Dog",
}

export interface DogPreview {
	thumbnail: Image,
	name: string,
	nickname: string,
	breed: string,
	gender: Gender,
	alive: boolean,
}

export interface Dog {
	thumbnail: Image,
	images: Image[],
	name: string,
	nickname: string,
	breed: string,
	gender: Gender,
	alive: boolean,
	dob: Date,
	// various extra details
}

export interface Image {
	src: string,
	alt: string,
}
