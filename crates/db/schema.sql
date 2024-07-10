CREATE TYPE breed AS enum ('Australian', 'Entlebuch', 'Bernese');
CREATE TYPE gender AS enum ('Male', 'Female');
CREATE TYPE availability AS enum ('Available', 'Unavailable', 'Co');

CREATE TABLE admins(
	id	serial	PRIMARY KEY,
	name	text	not null UNIQUE,
	salt	text	not null,
	hash	text	not null
);

CREATE TABLE dogs(
	id		serial 	PRIMARY KEY,
	thumbnail	text,
	name		text 	not null,
	nickname	text 	not null,
	father		text 	not null,
	mother		text	not null,
	dob		bigint	not null,
	breed		breed 	not null,
	gender		gender 	not null,
	alive		boolean	not null default true,
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
	litter_id	integer		references litters (id)
);

CREATE TABLE news(
	id		serial 	PRIMARY KEY,
	title		text 	not null,
	text		text 	not null,
	date		bigint	not null,
	images		text[] 	not null default '{}'
);
