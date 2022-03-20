-- ----------------------------------------------------------------
-- cd PATH
-- ----------------------------------------------------------------
SELECT is_folder
FROM node
WHERE "path" = ARRAY['usr'];

-- ----------------------------------------------------------------
-- cr PATH [DATA]
-- ----------------------------------------------------------------
WITH RECURSIVE rec AS (
	SELECT
		ARRAY['usr', 'non-bin', 'cuong', 'file-created'] AS "path",
		'Data for `file-created` file' IS NULL AS is_folder,
		'Data for `file-created` file' AS "data"
	UNION ALL
	SELECT "path"[:(array_upper("path", 1) - 1)], TRUE AS is_folder, NULL
	FROM rec
	WHERE array_upper("path", 1) > 1
)
INSERT INTO node("path", is_folder, "data")
SELECT "path", is_folder, "data"
FROM rec
ON CONFLICT ("path") DO NOTHING
RETURNING id, "path", is_folder, created_at;

-- create new folder.
INSERT INTO node ("path", is_folder, "data")
VALUES (ARRAY['usr', 'holistic', 'new-folder'], NULL IS NULL, NULL)
RETURNING id, "path", is_folder, "data", created_at;

-- create new file.
INSERT INTO node ("path", is_folder, "data")
VALUES (ARRAY['usr', 'holistic', 'new-folder', 'new-file'], 'New data file' IS NULL, 'New data file')
RETURNING id, "path", is_folder, "data", created_at;

-- ----------------------------------------------------------------
-- cat FILE_PATH
-- ----------------------------------------------------------------
SELECT "data"
FROM node
WHERE
	"path" = ARRAY['usr', 'holistic', 'new-folder', 'new-file']
	AND is_folder = false;

-- ----------------------------------------------------------------
-- ls FOLDER_PATH
-- ----------------------------------------------------------------
-- Get all files.
SELECT id, "path", is_folder, created_at, char_length("data") AS "size"
FROM node
WHERE
	NOT is_folder
	AND "path" @> ARRAY[]::text[]
	AND array_upper("path", 1) = coalesce(array_upper(ARRAY[]::text[], 1), 0) + 1
UNION ALL
	SELECT p.id, p."path", p.is_folder, p.created_at, c_calc."size"
	FROM
	    node p
	    JOIN LATERAL (
	        SELECT p.id, coalesce(sum(char_length(c."data")), 0) AS "size"
	        FROM node c
	        WHERE
	            NOT c.is_folder
	            AND c."path" @> p."path"
	            AND array_upper(c."path", 1) = array_upper(p."path", 1) + 1
	    ) c_calc ON p.id = c_calc.id
	WHERE
		is_folder
		AND p."path" @> ARRAY[]::text[]
		AND array_upper(p."path", 1) = coalesce(array_upper(ARRAY[]::text[], 1), 0) + 1;

-- ----------------------------------------------------------------
-- find NAME
-- ----------------------------------------------------------------
SELECT id, "path", is_folder, "data", created_at
FROM node
WHERE
	"path" @> ARRAY[]::TEXT[]
	AND array_upper("path", 1) > coalesce(array_upper(ARRAY[]::TEXT[], 1), 0)
	AND "path"[array_upper("path", 1)] LIKE '%e%';

-- ----------------------------------------------------------------
-- up PATH NAME [DATA]
-- ----------------------------------------------------------------
-- update folder name.
UPDATE node
SET
    "path" = ARRAY['usr', 'holistic-2']
		|| "path"[(coalesce(array_upper(ARRAY['usr', 'holistic'], 1), 0) + 1):]
WHERE "path" @> ARRAY['usr', 'holistic']
RETURNING id, "path", is_folder, "data", created_at;

-- update file data.
UPDATE node
SET
    "path" = ARRAY['usr', 'holistic', 'new-folder', 'new-file']
		|| "path"[((coalesce(array_upper(ARRAY['usr', 'holistic', 'new-folder', 'new-file'], 1), 0) + 1)):],
    "data" = 'Updated data'
WHERE "path" = ARRAY['usr', 'holistic', 'new-folder', 'new-file']
RETURNING id, "path", is_folder, "data", created_at;

-- ----------------------------------------------------------------
-- mv PATH FOLDER_PATH
-- ----------------------------------------------------------------
UPDATE node
SET
    "path" = ARRAY['share', 'lib']
		|| "path"[(coalesce(array_upper(ARRAY['usr', 'holistic-2'], 1), 0)):]
WHERE "path" @> ARRAY['usr', 'holistic-2']
RETURNING id, "path", is_folder, "data", created_at;

-- ----------------------------------------------------------------
-- rm PATH
-- ----------------------------------------------------------------
-- delete file.
DELETE FROM node
WHERE "path" @> ARRAY['share', 'lib', 'holistic-2', 'nothing']
RETURNING id, "path", is_folder, "data", created_at;

-- delete folder.
DELETE FROM node
WHERE "path" @> ARRAY['share', 'lib', 'holistic-2', 'new-folder']
RETURNING id, "path", is_folder, "data", created_at;