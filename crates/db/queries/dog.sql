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

--! dogs_by_breed_and_status : Dog()
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
	alive = :alive;

--! insert_dog
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
	:titles
);

--! update_dog
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
	images = :images,
	health = :health,
	titles = :titles
WHERE
	id = :id;

--! delete_dog
DELETE FROM
	dogs
WHERE
	id = :id;
