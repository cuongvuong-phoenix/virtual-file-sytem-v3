-- ----------------------------------------------------------------
-- `cd FOLDER_PATH` (just showing in the UI).
-- ----------------------------------------------------------------

-- ----------------------------------------------------------------
-- `cr PATH`
-- ----------------------------------------------------------------
CREATE OR REPLACE FUNCTION create_folder(varchar, varchar, varchar)
RETURNS TABLE(
	"path"      varchar,
	"type"      node_type,
	"name"      varchar,
	created_at  timestamptz
)
LANGUAGE sql
AS $$
    INSERT INTO node ("path", "type", "name", parent_path)
    VALUES ($1, 'folder', $2, $3)
    RETURNING "path", "type", "name", created_at;
$$;

-- 200.
SELECT * FROM create_folder('/usr/holistic', 'holistic', '/usr');

-- ----------------------------------------------------------------
-- `cr PATH DATA`
-- ----------------------------------------------------------------
CREATE OR REPLACE FUNCTION create_file(varchar, varchar, text, varchar)
RETURNS TABLE (
	"path"      varchar,
	"type"      node_type,
	"name"      varchar,
	created_at  timestamptz
)
LANGUAGE sql
AS $$
    INSERT INTO node ("path", "type", "name", "data", parent_path)
    VALUES ($1, 'file', $2, $3, $4)
    RETURNING "path", "type", "name", created_at;
$$

-- 200.
SELECT * FROM create_file('/usr/holistic/cuong', 'cuong', 'This is the `cuong` file', '/usr/holistic');

-- ----------------------------------------------------------------
-- `cat FILE_PATH`
-- ----------------------------------------------------------------
CREATE OR REPLACE FUNCTION read_file(varchar)
RETURNS text
LANGUAGE sql
AS $$
    SELECT "data" 
    FROM node
    WHERE "type" = 'file' AND "path" = $1;
$$

-- 200.
SELECT * FROM read_file('/bin/echo');
-- 404.
SELECT * FROM read_file('/non-exist-file');

-- ----------------------------------------------------------------
-- `ls [FOLDER_PATH]`
-- ----------------------------------------------------------------
CREATE OR REPLACE FUNCTION list_nodes(varchar)
RETURNS TABLE (
	PATH varchar,
	TYPE node_type,
	"name" varchar,
	created_at timestamptz,
	SIZE integer
)
LANGUAGE sql
AS $$
    -- List all files.
    SELECT "path", "type", "name", created_at, char_length("data") AS size
    FROM node
    WHERE "type" = 'file' AND parent_path = $1
    UNION
        -- List all folders.
        SELECT p."path", p."type", p."name", p.created_at, c.count AS size
        FROM
            node p
            JOIN LATERAL (
               SELECT p."path", count(*) FROM node c WHERE p."path" = c.parent_path AND c."type" = 'file'
            ) c
			ON p."path" = c."path"
        WHERE p."type" = 'folder' AND p.parent_path = $1;
$$

SELECT * FROM list_nodes('/usr');

-- ----------------------------------------------------------------
-- `up PATH NAME`
-- ----------------------------------------------------------------
CREATE OR REPLACE FUNCTION update_node_name_data(varchar, varchar, varchar, text)
RETURNS varchar
LANGUAGE sql
AS $$
    UPDATE node
    SET
        "path" = $2,
        "name" = $3,
        "data" = coalesce($4, "data")

    WHERE "path" = $1
    RETURNING "path";
$$

-- (200) Update folder name.
SELECT * FROM update_node_name_data('/usr/holistic', '/usr/holistic-2', 'holistic-2');
-- (200) Update file name.
SELECT * FROM update_node_name_data('/usr/holistic/cuong', '/usr/holistic-2', 'holistic-2');




DROP FUNCTION create_folder;
DROP FUNCTION create_file;
DROP FUNCTION list_nodes;