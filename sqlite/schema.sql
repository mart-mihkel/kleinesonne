create table dogs(
	id		integer not null primary key autoincrement,
	name		text not null,
	nickname	text not null,
	father		text not null,
	mother		text not null,
	thumbnail	text,
	dob		integer not null,
	breed		text not null  check(breed in ('aus', 'ent', 'ber')),
	gender		text not null  check(gender in ('male', 'female')),
	alive		integer not null  check(alive in (0, 1)),
	images		integer references dog_images(dog) on delete cascade,
	health		integer not null references health(dog) on delete cascade,
	titles		integer not null references titles(dog) on delete cascade
);

create table health(
	id		integer not null primary key autoincrement,
	dog		integer not null references dogs(id),
	health		text not null
);

create table dog_images(
	id		integer not null primary key autoincrement,
	dog		integer not null references dogs(id),
	path		text not null
);

create table litters(
	id		integer not null primary key autoincrement,
	name		text not null,
	parents		text,
	breed		text not null check(breed in ('aus', 'ent', 'ber')),
	puppies		integer references puppies(id) on delete cascade,
	images		integer references litter_images(litter) on delete cascade
);

create table puppies(
	id integer	not null primary key autoincrement,
	image text	,
	gender text	not null  check(gender in ('male', 'female')),
	availability	text not null check(availability in ('available', 'unavailable', 'co'))
);

create table litter_images(
	id		integer not null primary key autoincrement,
	litter		integer not null references litters(id),
	path		text not null
);

create table news(
	id		integer not null primary key autoincrement,
	title		text not null,
	text		text not null,
	date		integer not null,
	images		references news_images(piece) on delete cascade
);

create table news_images(
	id		integer not null primary key autoincrement,
	piece		integer not null references news(id),
	path		text not null
);
