-- ----------------------------------------------------------------
-- cr PATH [DATA]
-- ----------------------------------------------------------------
-- create new folder.
INSERT INTO node_mp_arr ("path", is_folder, "data")
VALUES (ARRAY['/', 'usr', 'holistic', 'new-folder'], NULL IS NULL, NULL)
RETURNING "path", is_folder, "data", created_at;

-- create new file.
INSERT INTO node_mp_arr ("path", is_folder, "data")
VALUES (ARRAY['/', 'usr', 'holistic', 'new-folder', 'new-file'], 'New data file' IS NULL, 'New data file')
RETURNING "path", is_folder, "data", created_at;

-- ----------------------------------------------------------------
-- cat FILE_PATH
-- ----------------------------------------------------------------
SELECT "path", is_folder, "data", created_at
FROM node_mp_arr
WHERE
	"path" = ARRAY['/', 'usr', 'holistic', 'new-folder', 'new-file']
	AND is_folder = false;

-- ----------------------------------------------------------------
-- ls FOLDER_PATH
-- ----------------------------------------------------------------
-- Get all files.
SELECT "path", is_folder, created_at, char_length("data") AS "size"
FROM node_mp_arr
WHERE
	NOT is_folder
	AND "path" @> ARRAY['/', 'share']
	AND array_length("path", 1) = array_length(ARRAY['/', 'share'], 1) + 1
UNION
	SELECT p."path", p.is_folder, p.created_at, c_calc."size"
	FROM
	    node_mp_arr p
	    JOIN LATERAL (
	        SELECT p."path", coalesce(sum(char_length(c."data")), 0) AS "size"
	        FROM node_mp_arr c
	        WHERE
	            NOT c.is_folder
	            AND c."path" @> p."path"
	            AND array_length(c."path", 1) = array_length(p."path", 1) + 1
	    ) c_calc ON p."path" = c_calc."path"
	WHERE
		is_folder
		AND p."path" @> ARRAY['/', 'share']
		AND array_length(p."path", 1) = array_length(ARRAY['/', 'share'], 1) + 1;

-- ----------------------------------------------------------------
-- find NAME
-- ----------------------------------------------------------------
SELECT "path", is_folder, "data", created_at
FROM node_mp_arr
WHERE
	"path" @> ARRAY['/', 'usr', 'holistic']
	AND array_length("path", 1) > array_length(ARRAY['/', 'usr', 'holistic'], 1)
	AND "path"[array_length("path", 1)] LIKE '%o%';

-- ----------------------------------------------------------------
-- up PATH NAME [DATA]
-- ----------------------------------------------------------------
-- update folder name.
UPDATE node_mp_arr
SET
    "path" = (ARRAY['/', 'usr', 'holistic-2'] || "path"[(array_length(ARRAY['/', 'usr', 'holistic'], 1) + 1):])
WHERE "path" @> ARRAY['/', 'usr', 'holistic']
RETURNING "path", is_folder, "data", created_at;

-- update file data.
UPDATE node_mp_arr
SET
    "path" = (ARRAY['/', 'usr', 'holistic', 'new-folder', 'new-file'] || "path"[(array_length(ARRAY['/', 'usr', 'holistic', 'new-folder', 'new-file'], 1) + 1):]),
    "data" = 'Updated data'
WHERE "path" = ARRAY['/', 'usr', 'holistic', 'new-folder', 'new-file']
RETURNING "path", is_folder, "data", created_at;

-- ----------------------------------------------------------------
-- mv PATH FOLDER_PATH
-- ----------------------------------------------------------------
UPDATE node_mp_arr
SET
    "path" = (ARRAY['/', 'share', 'lib', 'holistic-2'] || "path"[(array_length(ARRAY['/', 'usr', 'holistic-2'], 1) + 1):])
WHERE "path" @> ARRAY['/', 'usr', 'holistic-2']
RETURNING "path", is_folder, "data", created_at;

-- ----------------------------------------------------------------
-- rm PATH
-- ----------------------------------------------------------------
-- delete file.
DELETE FROM node_mp_arr
WHERE "path" @> ARRAY['/', 'share', 'lib', 'holistic-2', 'nothing']
RETURNING "path", is_folder, "data", created_at;

-- delete folder.
DELETE FROM node_mp_arr
WHERE "path" @> ARRAY['/', 'share', 'lib', 'holistic-2', 'new-folder']
RETURNING "path", is_folder, "data", created_at;
