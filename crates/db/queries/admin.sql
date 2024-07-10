--! get_by_name : Admin()
SELECT
	id,
	name,
	salt,
	hash
FROM
	admins
WHERE
	name = :name;
