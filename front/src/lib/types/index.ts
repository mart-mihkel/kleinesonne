export enum Gender {
	MALE,
	FEMALE,
}

export enum Breed {
	AUSTRALIAN = "Australian Shepherd",
	BERNESE = "Bernese Mountain Dog",
	ENTLEBUCHER = "Entlebucher Cattle Dog",
}

export interface Dog {
	img: Image,
	nickname: string,
	name: string,
	breed: string,
	gender: Gender,
	alive: boolean,
}

export interface Image {
	src: string,
	alt: string,
}
