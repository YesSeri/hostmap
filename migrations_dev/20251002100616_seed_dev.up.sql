-- hosts first (unchanged)
INSERT INTO host (hostname, metadata, host_url, created_at, updated_at)
VALUES
  ('test-1', '{"host_group_name":"test","env":"prod"}', 'nixos-test-1.local', NOW(), NOW()),
  ('test-2', '{"host_group_name":"test","env":"prod"}', 'nixos-test-2.local', NOW(), NOW()),
  ('test-3', '{"host_group_name":"test","env":"test"}', 'nixos-test-3.local', NOW(), NOW()),
  ('other-test-1', '{"host_group_name":"other-test","env":"prod"}', 'nixos-other-test-1.local', NOW(), NOW()),
  ('other-test-2', '{"host_group_name":"other-test","env":"prod"}', 'nixos-other-test-2.local', NOW(), NOW()),
  ('other-test-3', '{"host_group_name":"other-test","env":"test"}', 'nixos-other-test-3.local', NOW(), NOW())
ON CONFLICT (hostname) DO NOTHING;

-- ensure store paths exist before any FK use
INSERT INTO NixStorePath (store_path)
VALUES
  ('/nix/store/abcd1234'),
  ('/nix/store/efgh5678'),
  ('/nix/store/ijkl9012'),
  ('/nix/store/ndkj2312')
ON CONFLICT (store_path) DO NOTHING;

-- any log entries that mention store_path can now succeed (schema not shown)
INSERT INTO log_entry (timestamp, hostname, username, store_path, activation_type)
VALUES
  (NOW() - INTERVAL '1 hour',      'test-1',       'alice', '/nix/store/abcd1234', 'switch'),
  (NOW() - INTERVAL '30 minutes',  'test-1',       'alice', '/nix/store/efgh5678', 'boot'),
  (NOW() - INTERVAL '10 minutes',  'test-2',       'bob',   '/nix/store/ijkl9012', 'switch'),
  (NOW() - INTERVAL '40 minutes',  'test-2',       'bob',   '/nix/store/ijkl9012', 'switch'),
  (NOW() - INTERVAL '90 minutes',  'test-2',       'bob',   '/nix/store/ijkl9012', 'switch'),
  (NOW() - INTERVAL '90 minutes',  'test-3',       'bob',   '/nix/store/ijkl9012', 'switch'),
  (NOW() - INTERVAL '90 minutes',  'test-3',       'bob',   '/nix/store/ijkl9012', 'switch'),
  (NOW() - INTERVAL '1 hour',      'other-test-1', 'alice', '/nix/store/abcd1234', 'switch'),
  (NOW() - INTERVAL '30 minutes',  'other-test-1', 'alice', '/nix/store/efgh5678', 'boot'),
  (NOW() - INTERVAL '10 minutes',  'other-test-2', 'bob',   '/nix/store/ijkl9012', 'switch'),
  (NOW() - INTERVAL '40 minutes',  'other-test-2', 'bob',   '/nix/store/ijkl9012', 'switch'),
  (NOW() - INTERVAL '90 minutes',  'other-test-2', 'bob',   '/nix/store/ijkl9012', 'switch'),
  (NOW() - INTERVAL '90 minutes',  'other-test-3', 'bob',   '/nix/store/ijkl9012', 'switch'),
  (NOW() - INTERVAL '90 minutes',  'other-test-3', 'bob',   '/nix/store/ijkl9012', 'switch')
ON CONFLICT DO NOTHING;

-- finally, git-to-nix mapping; FKs now satisfied
INSERT INTO NixGitMapping (host_name, store_path, commit_hash, branch, deployed_at)
VALUES
  ('test-1',       '/nix/store/abcd1234', '3333cccc', 'main',    NOW() - INTERVAL '1 hour'),
  ('test-1',       '/nix/store/efgh5678', '2222bbbb', 'develop', NOW() - INTERVAL '30 minutes'),
  ('test-2',       '/nix/store/ndkj2312', '3333cccc', 'main',    NOW() - INTERVAL '10 minutes'),
  ('test-2',       '/nix/store/ndkj2312', '3333cccc', 'main',    NOW() - INTERVAL '20 minutes'),
  ('test-3',       '/nix/store/ijkl9012', '3333cccc', 'main',    NOW() - INTERVAL '10 minutes'),
  ('test-3',       '/nix/store/ijkl9012', '3333cccc', 'main',    NOW() - INTERVAL '20 minutes'),
  ('other-test-1', '/nix/store/abcd1234', '3333cccc', 'main',    NOW() - INTERVAL '1 hour'),
  ('other-test-1', '/nix/store/efgh5678', '2222bbbb', 'develop', NOW() - INTERVAL '30 minutes'),
  ('other-test-2', '/nix/store/ndkj2312', '3333cccc', 'main',    NOW() - INTERVAL '10 minutes'),
  ('other-test-2', '/nix/store/ndkj2312', '3333cccc', 'main',    NOW() - INTERVAL '20 minutes'),
  ('other-test-3', '/nix/store/ijkl9012', '3333cccc', 'main',    NOW() - INTERVAL '10 minutes'),
  ('other-test-3', '/nix/store/ijkl9012', '3333cccc', 'main',    NOW() - INTERVAL '20 minutes')
ON CONFLICT DO NOTHING;
