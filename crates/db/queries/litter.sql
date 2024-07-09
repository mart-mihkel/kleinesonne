--! get_litter_names
SELECT
	id,
	name
FROM
	litters;

--! get_litter : Litter()
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
	:images
);

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
