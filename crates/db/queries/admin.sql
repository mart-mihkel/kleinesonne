--! admin_exists
SELECT EXISTS(
	SELECT 
		id
	FROM
		admins
	WHERE
		name = :name
	AND
		hash = :hash
);
