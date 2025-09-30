-- Add up migration script here

-- platform-tools;/nix/store/gkc5227barafrwxwhxjcfvsmn10af1m0-nixos-system-platform-tools-25.05pre-git;9486ab0c9980e6c606bdc00356fac9702fe2d1f0;master;https://gitlab.dbc.dk/platform/deployments/-/pipelines/470359;2025-09-23 10:59:37+02:00
-- vault-a-p01;/nix/store/nv73i06dq06s2d4bx0rwav0vzfzhs5nc-nixos-system-vault-a-p01-25.05pre-git;9486ab0c9980e6c606bdc00356fac9702fe2d1f0;master;https://gitlab.dbc.dk/platform/deployments/-/pipelines/470359;2025-09-23 10:59:37+02:00
-- vault-a-p201;/nix/store/gh4pyn9dzxzp2magw545f1nshbpv82y4-nixos-system-vault-a-p201-25.05pre-git;9486ab0c9980e6c606bdc00356fac9702fe2d1f0;master;https://gitlab.dbc.dk/platform/deployments/-/pipelines/470359;2025-09-23 10:59:37+02:00
-- vault-a-p301;/nix/store/02hskpfc2gvr7qvzblyipkpmnyafhkw2-nixos-system-vault-a-p301-25.05pre-git;9486ab0c9980e6c606bdc00356fac9702fe2d1f0;master;https://gitlab.dbc.dk/platform/deployments/-/pipelines/470359;2025-09-23 10:59:37+02:00
-- vault-b-p01;/nix/store/l1baimfwz8hd6nl8q4jh4a57047clvyd-nixos-system-vault-b-p01-25.05pre-git;9486ab0c9980e6c606bdc00356fac9702fe2d1f0;master;https://gitlab.dbc.dk/platform/deployments/-/pipelines/470359;2025-09-23 10:59:37+02:00
-- vault-s01;/nix/store/zfpsd84sq9i9hy74s13pc1a0b2pfayw3-nixos-system-vault-s01-25.05pre-git;9486ab0c9980e6c606bdc00356fac9702fe2d1f0;master;https://gitlab.dbc.dk/platform/deployments/-/pipelines/470359;2025-09-23 10:59:37+02:00
-- vault-s02;/nix/store/iwwr3j1jbgks0hwan92d2rih10kl443b-nixos-system-vault-s02-25.05pre-git;9486ab0c9980e6c606bdc00356fac9702fe2d1f0;master;https://gitlab.dbc.dk/platform/deployments/-/pipelines/470359;2025-09-23 10:59:37+02:00
-- vault-s03;/nix/store/wnakf97vfnhl2y8bzhyd3mzwvm3ga9ia-nixos-system-vault-s03-25.05pre-git;9486ab0c9980e6c606bdc00356fac9702fe2d1f0;master;https://gitlab.dbc.dk/platform/deployments/-/pipelines/470359;2025-09-23 10:59:37+02:00
-- wharfix-p01;/nix/store/xi97xabnm0ifk70vpi3dq4nmqrv49r8w-nixos-system-wharfix-p01-25.05pre-git;9486ab0c9980e6c606bdc00356fac9702fe2d1f0;master;https://gitlab.dbc.dk/platform/deployments/-/pipelines/470359;2025-09-23 10:59:37+02:00
-- wharfix-p02;/nix/store/kwpfb5whvdvp05gnqdf6zzjmminy0w49-nixos-system-wharfix-p02-25.05pre-git;9486ab0c9980e6c606bdc00356fac9702fe2d1f0;master;https://gitlab.dbc.dk/platform/deployments/-/pipelines/470359;2025-09-23 10:59:37+02:00

CREATE TABLE NixGitMapping (
    -- log_entry_id BIGSERIAL PRIMARY KEY,
    -- timestamp TIMESTAMPTZ NOT NULL,
    -- hostname TEXT NOT NULL,
    -- username TEXT NOT NULL,
    -- store_path TEXT NOT NULL,
    -- activation_type TEXT NOT NULL,
    -- FOREIGN KEY (hostname)
    --     REFERENCES host(hostname)
    --     ON UPDATE CASCADE ON DELETE CASCADE,
	-- UNIQUE (hostname, username, timestamp, store_path, activation_type)
);
