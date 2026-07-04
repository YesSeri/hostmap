# hostmap

Hostmap tracks which NixOS system image is currently active on each host and links those running system images back to the Git commits that produced them.

It can be used by NixOS fleets where hosts are built and deployed from Git. Hostmap then enables easily seeing info such as:

- Which NixOS system image is currently active on this host?
- When did this host switch to a different system image?
- Which Git commit produced the running system image?
- What is the activation history of a host?
- What is the [nix-diff](https://github.com/Gabriella439/nix-diff) between these two system images, or commits.

## Overview


For a simpler introduction to hostmap, go to the [hostmap demo repo](https://github.com/yesseri/hostmap-demo). This repo starts three vms, and lets you try out what hostmap actually is.

There are three parts to hostmap.

- Server. Stores the activation(switching nix image) data and store-path-to-commit mappings. It also exposes this data through a website.
- Scraper. Scrapes the activation logs from the hosts and sends the data to the server.
- Activation logger. Runs on NixOS hosts and makes system activation information available for the scraper to fetch.

A fourth thing is needed, but that is not part of hostmap. A CI server, or other solution that builds the commits and sees what the resulting nix system image path is. This needs to be supplied by the user of the hostmap to the hostmap server.

## Usage


Enter the development shell:

```bash
nix develop
```

There are three commands:

```
cargo run -- --server
cargo run -- --scraper
cargo run -- --activation-logger
```

Start the server like this:

```
cargo run -- server \
  --database-url "$DATABASE_URL" \
  --api-key-file ./dev-api-key.txt \
  --repo-url "https://github.com/yesseri/hostmap-demo/commit"
```


Start the scraper like this:

```
cargo run scraper --host-group-file ./test-assets/minimalTargetList.json --scrape-interval 5
```

Start the activation logger like this:

```
cargo run -- activation-logger \
  --activation-log-file ./test-assets/activationlog.csv \
  --url-path /activationlog.csv \
  --port 9001
```





