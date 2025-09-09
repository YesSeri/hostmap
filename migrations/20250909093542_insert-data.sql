-- Add up migration script here
INSERT INTO host_groups (name) VALUES ('hosts-prod') ON CONFLICT (name) DO NOTHING;
-- INSERT INTO host_groups (name) VALUES ('devpi-prod') ON CONFLICT (name) DO NOTHING;
-- INSERT INTO host_groups (name) VALUES ('ceph-prod') ON CONFLICT (name) DO NOTHING;

INSERT INTO hosts (host_group_id, host, host_url, loc, system, rev, rev_url, ref_)
VALUES
    (
        (SELECT id FROM host_groups WHERE name='hosts-prod'),
        'hosts-p01',
        'hosts-p01.pzz.dk',
    ),
    (
        (SELECT id FROM host_groups WHERE name='ceph-prod'),
        'hosts-p02',
        'hosts-p02.pzz.dk',
    );

-- INSERT INTO hosts (host_group_id, host, host_url, loc, system, rev, rev_url, ref_)
-- VALUES
--     (
--         (SELECT id FROM host_groups WHERE name='artifactory-prod'),
--         'artifactory-p101',
--         'history.php@host=artifactory-p101.html',
--         'M1 K1',
--         '9nnn30n9dy-artifactory-p101-25.05',
--         '1c1d370d44ae5ee23a49acf968dfe10ecaff6700',
--         'https://gitlab.dbc.dk/platform/deployments/commit/1c1d370d44ae5ee23a49acf968dfe10ecaff6787',
--         'master'
--     ),
--     (
--         (SELECT id FROM host_groups WHERE name='artifactory-prod'),
--         'artifactory-p201',
--         'history.php@host=artifactory-p201.html',
--         'M2 K2',
--         'rn9wi0x5wy-artifactory-p201-25.05',
--         '1c1d370d44ae5ee23a49acf968dfe10ecaff6700',
--         'https://gitlab.dbc.dk/platform/deployments/commit/1c1d370d44ae5ee23a49acf968dfe10ecaff6787',
--         'master'
--     );
-- INSERT INTO hosts (host_group_id, host, host_url, loc, system, rev, rev_url, ref_)
-- VALUES
--     (
--         (SELECT id FROM host_groups WHERE name='devpi-prod'),
--         'devpi-p01',
--         'history.php@host=devpi-p01.html',
--         'VM',
--         'fig83z1j81-devpi-p01-25.05',
--         '1c1d370d44ae5ee23a49acf968dfe10ecaff87',
--         'https://gitlab.dbc.dk/platform/deployments/commit/1c1d370d44ae5ee23a49acf968dfe10ecaff6787',
--         'master'
--     );
