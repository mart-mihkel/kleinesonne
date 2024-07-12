--! all_names
SELECT
	id,
	name
FROM
	litters;

--! litter_by_id : Litter()
SELECT
	id,
	name,
	breed,
	parents_image,
	images
FROM
	litters
WHERE
	id = :id;

--! litters_by_breed : Litter()
SELECT
	id,
	name,
	breed,
	parents_image,
	images
FROM
	litters
WHERE
	id = :id and breed = :breed;

--! insert_litter
INSERT INTO
	litters(
		name,
		breed,
		parents_image,
		images
	)
VALUES(
	:name,
	:breed,
	:parents_image,
	:images)
RETURNING
	id;

--! update_litter
UPDATE
	litters
SET
	name = :name,
	breed = :breed,
	parents_image = :parents_image,
	images = :images
WHERE
	id = :id;

--! delete_litter
DELETE FROM
	litters
WHERE
	id = :id;
