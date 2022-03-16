DELETE FROM node_mp_arr;

INSERT INTO node_mp_arr ("path", is_folder, "data")
VALUES
    (ARRAY['/'], true, NULL),
    (ARRAY['/', 'usr'], true, NULL),
    (ARRAY['/', 'usr', 'bin'], true, NULL),
    (ARRAY['/', 'usr', 'bin', 'sh'], false, 'Data for `sh` file'),
    (ARRAY['/', 'usr', 'bin', 'echo'], false, 'Data for `sh` file'),
    (ARRAY['/', 'usr', 'holistic'], true, NULL),
    (ARRAY['/', 'usr', 'holistic', 'cuong'], true, NULL),
    (ARRAY['/', 'usr', 'holistic', 'cuong', 'a'], true, NULL),
    (ARRAY['/', 'usr', 'holistic', 'cuong', 'b'], true, NULL),
    (ARRAY['/', 'usr', 'holistic', 'cuong', 'b', 'hello'], false, 'Data for `hello` file'),
    (ARRAY['/', 'usr', 'holistic', 'nothing'], false, 'Data for `nothing` file'),
    (ARRAY['/', 'share'], true, NULL),
    (ARRAY['/', 'share', 'local'], true, NULL),
    (ARRAY['/', 'share', 'local', '1'], false, 'Data for `1` file'),
    (ARRAY['/', 'share', 'local', '2'], false, 'Data for `2` file'),
    (ARRAY['/', 'share', 'lib'], true, NULL)
;