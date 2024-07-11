--! get_n_news_older_than : Article()
SELECT
	id,
	title,
	text,
	date,
	images
FROM
	news
WHERE
	date < :date
LIMIT
	:n;

--! insert_news
INSERT INTO
	news(
		title,
		text,
		date,
		images
	)
VALUES(
	:title,
	:text,
	:date,
	:images)
RETURNING
	id;

--! update_news
UPDATE
	news
SET
	title = :title,
	text = :text,
	date = :date,
	images = :images
WHERE
	id = :id;

--! delete_news
DELETE FROM
	news
WHERE
	id = :id;
