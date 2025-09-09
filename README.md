#
run like this

```bash
nix develop
pg_setup
pg_ctl -D /PATH/TO/REPO/hostmap/.dev_postgres//data -l logfile start
DATABASE_URL="postgres://<USERNAME>:postgres@localhost:5432/hostmap-dev" cargo run -- test-assets/minimalTargetList.json
```