CREATE TYPE breed AS ENUM ('aus', 'ent', 'ber');
CREATE TYPE gender AS ENUM ('male', 'female');
CREATE TYPE availability AS ENUM ('available', 'unavailable', 'co');

CREATE TABLE admins(
	id	serial	PRIMARY KEY,
	name	text	not null,
	hash	text	not null
);

CREATE TABLE dogs(
	id		serial 	PRIMARY KEY,
	thumbnail	text,
	name		text 	not null,
	nickname	text 	not null,
	father		text 	not null,
	mother		text	not null,
	dob		date 	not null,
	breed		breed 	not null,
	gender		gender 	not null,
	alive		boolean	not null default 1,
	health		text[] 	not null default '{}',
	titles 		text[] 	not null default '{}',
	images		text[] 	not null default '{}'
);

CREATE TABLE litters(
	id		serial 	PRIMARY KEY,
	parents_image	text,
	name		text 	not null,
	breed		breed 	not null,
	images		text[] 	not null default '{}'
);

CREATE TABLE puppies(
	id		serial 		PRIMARY KEY,
	image		text,
	name		text 		not null,
	gender		gender 		not null,
	availability	availability	not null,
	litter_id	integer 	not null FOREIGN KEY REFERENCES litters(id) ON DELETE CASCADE,
);

CREATE TABLE news(
	id		serial 	PRIMARY KEY,
	title		text 	not null,
	text		text 	not null,
	date		date	not null,
	images		text[] 	not null default '{}'
);
