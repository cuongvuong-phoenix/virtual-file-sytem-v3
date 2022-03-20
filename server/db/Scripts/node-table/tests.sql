-- ----------------------------------------------------------------
-- Materialized Path using array with GIN index
-- ----------------------------------------------------------------
CREATE TABLE mp_arr (
	"path" text[] PRIMARY KEY
);

CREATE INDEX mp_arr_path_gin ON mp_arr USING gin ("path");

INSERT INTO mp_arr ("path")
VALUES
	(ARRAY['1']),
	(ARRAY['1', '1']),
	(ARRAY['1', '2']),
	(ARRAY['1', '2', '1']),
	(ARRAY['1', '3']),
	(ARRAY['1', '3', '1']),
	(ARRAY['1', '3', '1', '1'])
;

EXPLAIN SELECT *
FROM mp_arr
WHERE "path" @> ARRAY[1, 2];

-- ----------------------------------------------------------------
-- Materialized Path using string
-- ----------------------------------------------------------------
CREATE TABLE mp_str (
	"path" text PRIMARY KEY
);

INSERT INTO mp_str ("path")
VALUES
	('1'),
	('1/1'),
	('1/2'),
	('1/2/1'),
	('1/3'),
	('1/3/1'),
	('1/3/1/1')
;

EXPLAIN ANALYZE SELECT *
FROM mp_str
WHERE "path" LIKE '1/3/%' AND "path" NOT LIKE '1/3/%/%';