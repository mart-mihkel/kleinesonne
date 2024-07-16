--! all_names
SELECT
	id,
	name
FROM
	dogs;

--! dog_by_id : Dog()
SELECT
	id,
	name,
	nickname,
	father,
	mother,
	breed,
	gender,
	dob,
	alive,
	thumbnail,
	images,
	health,
	titles
FROM
	dogs
WHERE
	id = :id;

--! alive_dogs_by_breed_and_gender: Dog()
SELECT
	id,
	name,
	nickname,
	father,
	mother,
	breed,
	gender,
	dob,
	alive,
	thumbnail,
	images,
	health,
	titles
FROM
	dogs
WHERE
	breed = :breed
AND
	gender = :gender
AND
	alive = true;

--! retired_dogs_by_breed: Dog()
SELECT
	id,
	name,
	nickname,
	father,
	mother,
	breed,
	gender,
	dob,
	alive,
	thumbnail,
	images,
	health,
	titles
FROM
	dogs
WHERE
	breed = :breed
AND
	alive = false;

--! insert_dog (thumbnail?)
INSERT INTO
	dogs(
		name,
		nickname,
		father,
		mother,
		breed,
		gender,
		dob,
		alive,
		thumbnail,
		images,
		health,
		titles
	)
VALUES(
	:name,
	:nickname,
	:father,
	:mother,
	:breed,
	:gender,
	:dob,
	:alive,
	:thumbnail,
	:images,
	:health,
	:titles)
RETURNING
	id;

--! update_dog (thumbnail?)
UPDATE
	dogs
SET
	name = :name,
	nickname = :nickname,
	father = :father,
	mother = :mother,
	breed = :breed,
	gender = :gender,
	dob = :dob,
	alive = :alive,
	thumbnail = :thumbnail,
	images = array_cat(images, :images),
	health = :health,
	titles = :titles
WHERE
	id = :id;

--! delete_dog
DELETE FROM
	dogs
WHERE
	id = :id;
