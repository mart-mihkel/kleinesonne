--! get_by_litter : Litter()
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

--! insert_puppy
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

--! update_puppy
UPDATE
	puppies
SET
	litter_id = :litter_id,
	name = :name,
	gender = :gender,
	availability = :availability,
	image = :thumbnail
WHERE
	id = :id;

--! delete_puppy
DELETE FROM
	puppies
WHERE
	id = :id;

