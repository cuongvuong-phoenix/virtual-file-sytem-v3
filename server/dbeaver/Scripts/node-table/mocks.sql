INSERT INTO node ("path", "type", "name", "data", parent_path)
VALUES
    -- /
    ('/', 'folder', '/', NULL, NULL),
    -- /bin
    ('/bin', 'folder', 'bin', NULL, '/'),
    -- /bin/*
    ('/bin/arch', 'file', 'arch', 'This is the `arch` file', '/bin'),
    ('/bin/base64', 'file', 'base64', 'This is the `base64` file', '/bin'),
    ('/bin/bbconfig', 'file', 'bbconfig', 'This is the `bbconfig` file', '/bin'),
    ('/bin/cat', 'file', 'cat', 'This is the `cat` file', '/bin'),
    ('/bin/chgrp', 'file', 'chgrp', 'This is the `chgrp` file', '/bin'),
    ('/bin/chmod', 'file', 'chmod', 'This is the `chmod` file', '/bin'),
    ('/bin/chown', 'file', 'chown', 'This is the `chown` file', '/bin'),
    ('/bin/echo', 'file', 'echo', 'This is the `echo` file', '/bin'),
    -- /etc
    ('/etc', 'folder', 'etc', NULL, '/'),
    -- /etc/crontabs
    ('/etc/crontabs', 'folder', 'crontabs', NULL, '/etc'),
    -- /etc/crontabs/*
    ('/etc/crontabs/root', 'file', 'root',  'This is the `root` file', '/etc/crontabs'),
    -- /etc/network
    ('/etc/network', 'folder', 'network', NULL, '/etc'),
    -- /etc/network/if-up_d
    ('/etc/network/if-up_d', 'folder', 'if-up_d', NULL, '/etc/network'),
    -- /etc/network/if-up_d/*
    ('/etc/network/if-up_d/dad', 'file', 'dad',  'This is the `dad` file', '/etc/network/if-up_d'),
    -- /etc/network/if-down_d
    ('/etc/network/if-down_d', 'folder', 'if-down_d', NULL, '/etc/network'),
    -- /usr
    ('/usr', 'folder', 'usr', NULL, '/'),
    -- /usr/lib
    ('/usr/lib', 'folder', 'lib', NULL, '/usr'),
    -- /usr/lib/*
    ('/usr/lib/libcrypto_so_1_1', 'file', 'libcrypto_so_1_1',  'This is the `libcrypto_so_1_1` file', '/usr/lib'),
    ('/usr/lib/libssl_so_1_1', 'file', 'libssl_so_1_1',  'This is the `libssl_so_1_1` file', '/usr/lib'),
    ('/usr/lib/libtls_so_2', 'file', 'libtls_so_2',  'This is the `libtls_so_2` file', '/usr/lib'),
    ('/usr/lib/libtls_so_2_3', 'file', 'libtls_so_2_3',  'This is the `libtls_so_2_3` file', '/usr/lib'),
    -- /usr/share
    ('/usr/share', 'folder', 'share', NULL, '/usr'),
    -- /usr/share/apk
    ('/usr/share/apk', 'folder', 'apk', NULL, '/usr/share'),
    -- /usr/share/apk/keys
    ('/usr/share/apk/keys', 'folder', 'keys', NULL, '/usr/share/apk'),
    -- /usr/share/apk/keys/*
    ('/usr/share/apk/keys/alpine-devel', 'file', 'alpine-devel',  'This is the `alpine-devel` file', '/usr/share/apk/keys'),
    -- /usr/share/apk/keys/x86
    ('/usr/share/apk/keys/x86', 'folder', 'x86', NULL, '/usr/share/apk/keys'),
    -- /usr/share/apk/keys/x86/*
    ('/usr/share/apk/keys/x86/alpine-devel', 'file', 'alpine-devel',  'This is the `alpine-devel` file', '/usr/share/apk/keys/x86')
;
