INSERT INTO node_mp ("path", is_folder, "name", "data")
VALUES
    ('/', true, NULL, NULL),
    ('/usr', true, 'usr', NULL),
    ('/usr/bin', true, 'bin', NULL),
    ('/usr/bin/sh', false, 'sh', 'Data for `sh` file'),
    ('/usr/bin/echo', false, 'echo', 'Data for `sh` file'),
    ('/usr/holistic', true, 'holistic', NULL),
    ('/usr/holistic/cuong', true, 'cuong', NULL),
    ('/usr/holistic/cuong/a', true, 'a', NULL),
    ('/usr/holistic/cuong/b', true, 'b', NULL),
    ('/usr/holistic/cuong/b/hello', false, 'hello', 'Data for `hello` file'),
    ('/usr/holistic/nothing', false, 'nothing', 'Data for `nothing` file'),
    ('/share', true, 'share', NULL),
    ('/share/local', true, 'local', NULL),
    ('/share/local/1', false, '1', 'Data for `1` file'),
    ('/share/local/2', false, '2', 'Data for `2` file'),
    ('/share/lib', true, 'lib', NULL)
;