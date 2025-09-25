#
run like this

```bash
nix develop
pg_setup
pg_ctl -D /PATH/TO/REPO/hostmap/.dev_postgres//data -l logfile start
DATABASE_URL="postgres://<USERNAME>:postgres@localhost:5432/hostmap-dev" cargo run -- test-assets/minimalTargetList.json
```

```
run server:
cargo run server --database-url 'postgres://heze:postgres@localhost:5432/hostmap-dev'

run scraper:
cargo run scraper --host-group-file ./test-assets/minimalTargetList.json --scrape-interval 5
```
