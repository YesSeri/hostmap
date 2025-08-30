-- Add up migration script here
INSERT INTO host_groups (name) VALUES ('artifactory-prod') ON CONFLICT (name) DO NOTHING;
INSERT INTO host_groups (name) VALUES ('devpi-prod') ON CONFLICT (name) DO NOTHING;
INSERT INTO host_groups (name) VALUES ('ceph-prod') ON CONFLICT (name) DO NOTHING;

INSERT INTO hosts (host_group_id, host, host_url, loc, system, rev, rev_url, ref_)
VALUES
    (
        (SELECT id FROM host_groups WHERE name='ceph-prod'),
        'ceph-mon-p101',
        'history.php@host=ceph-mon-p101.html',
        'M1 K1',
        '6izsmg0r2d-ceph-mon-p101-25.05',
        'db9cbeea3f9ee49267ca9a648927f012633752e0',
        'https://gitlab.dbc.dk/platform/deployments/commit/db9cbeea3f9ee49267ca9a648927f012633752e0',
        'master'
    ),
    (
        (SELECT id FROM host_groups WHERE name='ceph-prod'),
        'ceph-mon-p201',
        'history.php@host=ceph-mon-p201.html',
        'M2 K2',
        '0dxcqdncfg-ceph-mon-p201-25.05',
        'db9cbeea3f9ee49267ca9a648927f012633752e0',
        'https://gitlab.dbc.dk/platform/deployments/commit/db9cbeea3f9ee49267ca9a648927f012633752e0',
        'master'
    );

INSERT INTO hosts (host_group_id, host, host_url, loc, system, rev, rev_url, ref_)
VALUES
    (
        (SELECT id FROM host_groups WHERE name='artifactory-prod'),
        'artifactory-p101',
        'history.php@host=artifactory-p101.html',
        'M1 K1',
        '9nnn30n9dy-artifactory-p101-25.05',
        '1c1d370d44ae5ee23a49acf968dfe10ecaff6700',
        'https://gitlab.dbc.dk/platform/deployments/commit/1c1d370d44ae5ee23a49acf968dfe10ecaff6787',
        'master'
    ),
    (
        (SELECT id FROM host_groups WHERE name='artifactory-prod'),
        'artifactory-p201',
        'history.php@host=artifactory-p201.html',
        'M2 K2',
        'rn9wi0x5wy-artifactory-p201-25.05',
        '1c1d370d44ae5ee23a49acf968dfe10ecaff6700',
        'https://gitlab.dbc.dk/platform/deployments/commit/1c1d370d44ae5ee23a49acf968dfe10ecaff6787',
        'master'
    );
INSERT INTO hosts (host_group_id, host, host_url, loc, system, rev, rev_url, ref_)
VALUES
    (
        (SELECT id FROM host_groups WHERE name='devpi-prod'),
        'devpi-p01',
        'history.php@host=devpi-p01.html',
        'VM',
        'fig83z1j81-devpi-p01-25.05',
        '1c1d370d44ae5ee23a49acf968dfe10ecaff87',
        'https://gitlab.dbc.dk/platform/deployments/commit/1c1d370d44ae5ee23a49acf968dfe10ecaff6787',
        'master'
    );
