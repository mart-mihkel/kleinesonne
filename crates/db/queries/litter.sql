--! all_names
SELECT
	id,
	name
FROM
	litters;

--! litter_by_id : Litter(parents_image?)
SELECT
	id,
	name,
	dob,
	breed,
	parents_image,
	images
FROM
	litters
WHERE
	id = :id;

--! available_litters : Litter(parents_image?)
SELECT
	id,
	name,
	dob,
	breed,
	parents_image,
	images
FROM
	litters l
WHERE
	(SELECT count(id) FROM puppies WHERE litter_id = l.id and availability in ('Available', 'Co')) > 0;

--! available_litters_by_breed : Litter(parents_image?)
SELECT
	id,
	name,
	dob,
	breed,
	parents_image,
	images
FROM
	litters l
WHERE
	breed = :breed
AND
	(SELECT count(id) FROM puppies WHERE litter_id = l.id and availability in ('Available', 'Co')) > 0;

--! insert_litter (parents_image?)
INSERT INTO
	litters(
		name,
		dob,
		breed,
		parents_image,
		images
	)
VALUES(
	:name,
	:dob,
	:breed,
	:parents_image,
	:images)
RETURNING
	id;

--! update_litter (parents_image?)
UPDATE
	litters
SET
	name = :name,
	dob = :dob,
	breed = :breed,
	parents_image = :parents_image,
	images = array_cat(images, :images)
WHERE
	id = :id;

--! delete_litter_parents_image
UPDATE
	litters
SET
	parents_image = null
WHERE
	id = :id;

--! delete_litter_image
UPDATE
	litters
SET
	images = array_remove(images, :image)
WHERE
	id = :id;


--! delete_litter
DELETE FROM
	litters
WHERE
	id = :id;
