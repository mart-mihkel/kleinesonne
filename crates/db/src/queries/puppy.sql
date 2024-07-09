--! get_by_litter
SELECT
	id,
	litter_id,
	name,
	gender,
	availability,
	thumbnail
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
		thumbnail
	)
VALUES(
	:litter_id,
	:name,
	:gender,
	:availability,
	:thumbnail
);

--! update_puppy
UPDATE
	puppies
SET
	litter_id = :litter_id,
	name = :name,
	gender = :gender,
	availability = :availability,
	thumbnail = :thumbnail
WHERE
	id = :id;

--! delete_puppy
DELETE FROM
	puppies
WHERE
	id = :id;
	
