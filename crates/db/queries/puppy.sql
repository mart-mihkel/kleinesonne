--! names_by_litter
SELECT
	id,
	name
FROM
	puppies
WHERE
	litter_id = :litter_id;

--! puppy_by_id : Puppy()
SELECT
	id,
	litter_id,
	name,
	gender,
	availability,
	image
FROM
	puppies
WHERE
	id = :id;


--! puppies_by_litter : Puppy()
SELECT
	id,
	litter_id,
	name,
	gender,
	availability,
	image
FROM
	puppies
WHERE
	litter_id = :litter_id;

--! avaliable_puppies_by_litter : Puppy()
SELECT
	id,
	litter_id,
	name,
	gender,
	availability,
	image
FROM
	puppies
WHERE
	litter_id = :litter_id and availability = 'Available';

--! insert_puppy (image?)
INSERT INTO
	puppies(
		litter_id,
		name,
		gender,
		availability,
		image
	)
VALUES(
	:litter_id,
	:name,
	:gender,
	:availability,
	:image)
RETURNING
	id;

--! update_puppy (image?)
UPDATE
	puppies
SET
	litter_id = :litter_id,
	name = :name,
	gender = :gender,
	availability = :availability,
	image = :image
WHERE
	id = :id;

--! delete_puppy
DELETE FROM
	puppies
WHERE
	id = :id;

