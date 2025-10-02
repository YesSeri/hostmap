BEGIN;

DELETE FROM NixGitMapping
WHERE commit_hash IN ('2222bbbb','3333cccc')
   OR store_path IN ('/nix/store/abcd1234','/nix/store/efgh5678','/nix/store/ijkl9012','/nix/store/ndkj2312')
   OR host_name IN ('test-1','test-2','test-3','other-test-1','other-test-2','other-test-3');

DELETE FROM log_entry
WHERE hostname IN ('test-1','test-2','test-3','other-test-1','other-test-2','other-test-3')
   OR store_path IN ('/nix/store/abcd1234','/nix/store/efgh5678','/nix/store/ijkl9012','/nix/store/ndkj2312');

DELETE FROM NixStorePath
WHERE store_path IN ('/nix/store/abcd1234','/nix/store/efgh5678','/nix/store/ijkl9012','/nix/store/ndkj2312');

DELETE FROM host
WHERE hostname IN ('test-1','test-2','test-3','other-test-1','other-test-2','other-test-3');

COMMIT;
