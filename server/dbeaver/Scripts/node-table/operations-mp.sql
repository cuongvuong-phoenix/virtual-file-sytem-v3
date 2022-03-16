-- ----------------------------------------------------------------
-- cr PATH
-- ----------------------------------------------------------------
INSERT INTO node_mp ("path", is_folder, "name")
VALUES ('/usr/holistic/new-folder', true, 'new-folder');

-- ----------------------------------------------------------------
-- cr PATH DATA
-- ----------------------------------------------------------------
INSERT INTO node_mp ("path", is_folder, "name", "data")
VALUES ('/usr/holistic/new-folder/new-file', false, 'new-file', 'LOL');

-- ----------------------------------------------------------------
-- cat FILE_PATH
-- ----------------------------------------------------------------
SELECT "data"
FROM node_mp
WHERE "path" = '/usr/holistic/new-folder/new-file' AND is_folder = false;

-- ----------------------------------------------------------------
-- ls FOLDER_PATH
-- ----------------------------------------------------------------
SELECT "path", is_folder, "name", "data", "created_at"
FROM node_mp
WHERE "path" LIKE '/usr/holistic/%' AND "path" NOT LIKE '/usr/holistic/%/%';

-- ----------------------------------------------------------------
-- find NAME
-- ----------------------------------------------------------------
SELECT "path", is_folder, "name", "data", "created_at"
FROM node_mp
WHERE "path" LIKE '/usr/holistic/%o%' AND "name" LIKE '%o%';

-- ----------------------------------------------------------------
-- up PATH NAME
-- ----------------------------------------------------------------
UPDATE node_mp
SET
    "path" = replace("path", '/usr/holistic', '/usr/holistic-2')
WHERE "path" LIKE '/usr/holistic%'
RETURNING "path", is_folder, "name", "data", "created_at";

UPDATE node_mp
SET "name" = 'holistic-2'
WHERE "path" = '/usr/holistic-2'
RETURNING "path", is_folder, "name", "data", "created_at";

-- ----------------------------------------------------------------
-- up PATH NAME DATA
-- ----------------------------------------------------------------
UPDATE node_mp
SET
    "path" = replace("path", '/usr/holistic-2/nothing', '/usr/holistic-2/nothing-with-new-data'),
    "name" = 'nothing-with-new-data',
    "data" = 'New data'
WHERE "path" = '/usr/holistic-2/nothing'
RETURNING "path", is_folder, "name", "data", "created_at";

-- ----------------------------------------------------------------
-- mv PATH FOLDER_PATH
-- ----------------------------------------------------------------
UPDATE node_mp
SET "path" = replace("path", '/usr', '/share/lib')
WHERE "path" LIKE '/usr/holistic-2%'
RETURNING "path", is_folder, "name", "data", "created_at";

-- ----------------------------------------------------------------
-- rm PATH
-- ----------------------------------------------------------------
DELETE FROM node_mp
WHERE "path" LIKE '/share/lib/holistic-2/nothing-with-new-data%'
RETURNING "path", is_folder, "name", "data", "created_at";

-- ----------------------------------------------------------------
-- Get all ancestors
-- ----------------------------------------------------------------
SELECT "path", is_folder, "name", "data", "created_at"
FROM node_mp
WHERE '/share/lib/holistic-2/cuong/b/hello' LIKE "path" || '%';

SELECT ARRAY[1,4,3] && ARRAY[2,1];



