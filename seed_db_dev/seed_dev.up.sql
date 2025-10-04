BEGIN TRANSACTION;
INSERT INTO host (hostname, metadata, host_url, created_at, updated_at)
VALUES
  ('test-1', '{"host_group_name":"test","env":"prod"}', 'nixos-test-1.local', NOW(), NOW()),
  ('test-2', '{"host_group_name":"test","env":"prod"}', 'nixos-test-2.local', NOW(), NOW()),
  ('test-3', '{"host_group_name":"test","env":"test"}', 'nixos-test-3.local', NOW(), NOW()),
  ('other-test-1', '{"host_group_name":"other-test","env":"prod"}', 'nixos-other-test-1.local', NOW(), NOW()),
  ('other-test-2', '{"host_group_name":"other-test","env":"prod"}', 'nixos-other-test-2.local', NOW(), NOW()),
  ('other-test-3', '{"host_group_name":"other-test","env":"test"}', 'nixos-other-test-3.local', NOW(), NOW())
ON CONFLICT (hostname) DO NOTHING;

INSERT INTO nix_store_path (store_path)
VALUES
  ('/nix/store/abcd1234'),
  ('/nix/store/efgh5678'),
  ('/nix/store/ffff1111'),
  ('/nix/store/ijkl9012'),
  ('/nix/store/ndkj2312'),
  ('/nix/store/xfkl9012'),
  ('/nix/store/xyz89012'),
  ('/nix/store/xxx89012'),
  ('/nix/store/yyy89012'),
  ('/nix/store/zzz89012') 

ON CONFLICT (store_path) DO NOTHING;

INSERT INTO activation(activated_at, hostname, username, store_path, activation_type)
VALUES
-- don't use now, use fixed times to make tests reliable
  ('2023-03-01 12:30:00',  'test-1',       'alice', '/nix/store/efgh5678', 'boot'),
  ('2023-03-01 12:00:00',  'test-1',       'alice', '/nix/store/abcd1234', 'switch'),
  ('2023-03-01 11:30:00',  'test-1',       'alice', '/nix/store/ffff1111', 'switch'),
  ('2023-03-01 11:10:00',  'test-2',       'bob',   '/nix/store/ndkj2312', 'switch'),
  ('2023-03-01 10:40:00',  'test-2',       'bob',   '/nix/store/ndkj2312', 'switch'),
  ('2023-03-01 10:20:00',  'test-2',       'bob',   '/nix/store/ndkj2312', 'switch'),
  ('2023-03-01 10:40:00',  'test-3',       'bob',   '/nix/store/ijkl9012', 'switch'),
  ('2023-03-01 09:40:00',  'test-3',       'bob',   '/nix/store/ijkl9012', 'switch'),
  ('2023-03-01 08:30:00',  'other-test-1', 'alice', '/nix/store/zzz89012', 'boot'),
  ('2023-03-01 07:30:00',  'other-test-1', 'alice', '/nix/store/yyy89012', 'switch'),
  ('2023-03-01 08:50:00',  'other-test-2', 'bob',   '/nix/store/xfkl9012', 'switch'),
  ('2023-03-01 08:10:00',  'other-test-2', 'bob',   '/nix/store/xfkl9012', 'switch'),
  ('2023-03-01 06:35:00',  'other-test-2', 'bob',   '/nix/store/xfkl9012', 'switch'),
  ('2023-03-01 06:30:00',  'other-test-3', 'bob',   '/nix/store/xyz89012', 'switch'),
  ('2023-03-01 05:30:00',  'other-test-3', 'bob',   '/nix/store/xyz89012', 'switch')
ON CONFLICT DO NOTHING;

INSERT INTO nix_git_link (store_path, commit_hash, branch, linked_at)
VALUES
  ('/nix/store/efgh5678', '00latest', 'master',     '2025-10-01 10:00:00'),
  ('/nix/store/efgh5678', '1111aaaa', 'develop',    '2025-10-01 09:00:00'),
  ('/nix/store/efgh5678', '1111aaaa', 'master',     '2025-10-01 08:45:00'),
  ('/nix/store/efgh5678', '3CORRECT', 'master',     '2025-10-01 08:30:00'),
  ('/nix/store/efgh5678', '555WRONG', 'feat/xyz',   '2025-10-01 08:20:00'),
  ('/nix/store/ffff1111', '444WRONG', 'master',     '2025-10-01 08:10:00'),
  ('/nix/store/ndkj2312', '3333cccc', 'master',     '2025-10-01 10:30:00'),
  ('/nix/store/ndkj2312', '4444cccc', 'master',     '2025-10-01 09:30:00'),
  ('/nix/store/ijkl9012', '3333cccc', 'master',     '2025-10-01 09:00:00'),
  ('/nix/store/ijkl9012', '3333cccc', 'master',     '2025-10-01 08:00:00'),
  ('/nix/store/zzz89012', '3333cccc', 'master',     '2025-10-01 07:00:00'),
  ('/nix/store/zzz89012', '2222bbbb', 'develop',    '2025-10-01 06:00:00'),
  ('/nix/store/xxx89012', '3333cccc', 'master',     '2025-10-01 06:30:00'),
  ('/nix/store/xxx89012', '3333cccc', 'master',     '2025-10-01 05:30:00'),
  ('/nix/store/yyy89012', '3333cccc', 'master',     '2025-10-01 05:00:00'),
  ('/nix/store/yyy89012', '3333cccc', 'master',     '2025-10-01 04:00:00'),
  ('/nix/store/xfkl9012', '2222bbbb', 'master',     '2025-10-01 07:30:00')
ON CONFLICT DO NOTHING;
COMMIT;