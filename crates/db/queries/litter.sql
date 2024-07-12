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

--! available_litters_by_breed : Litter()
SELECT
	id,
	name,
	breed,
	parents_image,
	images
FROM
	litters l
WHERE
	breed = :breed
AND
	(SELECT count(id) FROM puppies WHERE litter_id = l.id and availability = 'Available') > 0;

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
