DELETE FROM node;

INSERT INTO node ("path", is_folder, "data")
VALUES
    (ARRAY[]::TEXT[], true, NULL),
    (ARRAY['usr'], true, NULL),
    (ARRAY['usr', 'bin'], true, NULL),
    (ARRAY['usr', 'bin', 'sh'], false, 'Data for `sh` file'),
    (ARRAY['usr', 'bin', 'echo'], false, 'Data for `sh` file'),
    (ARRAY['usr', 'some-usr'], true, NULL),
    (ARRAY['usr', 'some-usr', 'some-folder'], true, NULL),
    (ARRAY['usr', 'some-usr', 'some-folder', 'a'], true, NULL),
    (ARRAY['usr', 'some-usr', 'some-folder', 'b'], true, NULL),
    (ARRAY['usr', 'some-usr', 'some-folder', 'b', 'hello'], false, 'Data for `hello` file'),
    (ARRAY['usr', 'some-usr', 'nothing here'], false, 'Data for `nothing` file'),
    (ARRAY['local'], true, NULL),
    (ARRAY['local', 'share'], true, NULL),
    (ARRAY['local', 'share', '1'], false, 'Data for `1` file'),
    (ARRAY['local', 'share', '2'], false, 'Data for `2` file'),
    (ARRAY['local', 'lib'], true, NULL)
;