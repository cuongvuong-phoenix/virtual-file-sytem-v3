CREATE TABLE node (
    id          serial PRIMARY KEY,
    "path"      text[] UNIQUE NOT NULL,
    is_folder   boolean NOT NULL,
    "data"      text,
    created_at  timestamptz NOT NULL DEFAULT current_timestamp
);

CREATE INDEX node_path_gin ON node USING gin ("path");