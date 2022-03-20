CREATE TABLE node (
    id          serial PRIMARY KEY,
    "path"      text[] UNIQUE NOT NULL,
    is_folder   boolean NOT NULL,
    "data"      text,
    created_at  timestamptz NOT NULL DEFAULT current_timestamp,

    CONSTRAINT valid_name CHECK ("path"[array_upper("path", 1)] ~  '^[a-zA-Z0-9 _-]+$')
);

CREATE INDEX node_path_gin ON node USING gin ("path");