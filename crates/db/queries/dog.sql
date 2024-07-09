--! dog_by_id
SELECT
	id,
	name,
	nickname,
	father,
	moter,
	breed,
	gender,
	dob,
	alive,
	thumnail,
	images,
	health,
	titles
FROM
	dogs
WHERE
	id = :id;

--! dogs_by_breed_and_status
SELECT
	id,
	name,
	nickname,
	father,
	moter,
	breed,
	gender,
	dob,
	alive,
	thumnail,
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
		moter,
		breed,
		gender,
		dob,
		alive,
		thumnail,
		images,
		health,
		titles
	)
VALUES(
	:name,
	:nickname,
	:father,
	:moter,
	:breed,
	:gender,
	:dob,
	:alive,
	:thumnail,
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
	mother = :moter,
	breed = :breed,
	gender = :gender,
	dob = :dob,
	alive = :alive,
	thumbnail = :thumnail,
	images = :images,
	health = :health,
	titles = :titles,
WHERE
	id = :id;

--! delete_dog
DELETE FROM
	dogs
WHERE
	id = :id;
