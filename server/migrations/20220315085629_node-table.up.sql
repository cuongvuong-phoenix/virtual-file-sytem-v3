CREATE TYPE node_type AS ENUM ('folder', 'file');

CREATE TABLE node (
    "path"          varchar PRIMARY KEY,
    "type"          node_type NOT NULL,
    "name"          varchar NOT NULL,
    "data"          text,
    created_at      timestamptz DEFAULT current_timestamp,
    parent_path     varchar REFERENCES node ("path") ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE node_mp (
    "path"      text PRIMARY KEY,
    is_folder   boolean NOT NULL,
    "name"      varchar,
    "data"      text,
    created_at  timestamptz DEFAULT current_timestamp
);

CREATE TABLE node_mp_arr (
    "path"      text[] PRIMARY KEY,
    is_folder   boolean NOT NULL,
    "data"      text,
    created_at  timestamptz DEFAULT current_timestamp
);

CREATE FUNCTION into_text_immutable(text[])
RETURNS text
LANGUAGE sql IMMUTABLE
AS $$
    SELECT $1::text
$$

CREATE INDEX node_mp_arr_path_gin ON node_mp_arr USING gin ("path");