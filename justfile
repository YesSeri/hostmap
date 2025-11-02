# ===== Settings =====
set dotenv-load := true                  # auto-load .env if present
set export := true                       # export variables to commands
set shell := ["bash", "-euo", "pipefail", "-c"]

# ===== Variables =====
CARGO := env_var_or_default("CARGO", "cargo")
DATABASE_URL := env_var("DATABASE_URL")
API_KEY_FILE := env_var_or_default("API_KEY_FILE", "./api-key.txt")
GROUPING_KEY := env_var_or_default("GROUPING_KEY", "host_group_name")
TARGETS_FILE := env_var_or_default("TARGETS_FILE", "test-assets/minimalTargetList.json")

# ===== Default =====
default:
  @just --list

# ===== Server =====
# Run the API server. Override grouping with: `just server key=my_key`
server key=GROUPING_KEY *args:
  @{{CARGO}} run -- server \
    --database-url "{{DATABASE_URL}}" \
    --api-key-file "{{API_KEY_FILE}}" \
    --grouping-key "{{key}}" \
    {{args}}

# ===== Scraper =====
# Run the scraper. Override via: `just scraper interval=60 concurrent=4 port=8080 hosts=path.json`
scraper interval="2" concurrent="4" port="80" hosts=TARGETS_FILE *args:
  @{{CARGO}} run -- scraper \
    --hosts-file "{{hosts}}" \
    --scrape-interval "{{interval}}" \
    --concurrent-requests "{{concurrent}}" \
    --activation-logger-port "{{port}}" \
    --api-key-file "{{API_KEY_FILE}}" \
    {{args}}

# ===== Build & QA =====
# Debug build
build *args:
  @{{CARGO}} build {{args}}

# Release build
release *args:
  @{{CARGO}} build --release {{args}}

# Format code
fmt:
  @{{CARGO}} fmt

# Lints
clippy *args:
  @{{CARGO}} clippy --all-targets -- -D warnings {{args}}

# Run tests
test *args:
  @{{CARGO}} test {{args}}

# ===== Aliases =====
alias b := build
alias r := release
alias s := server
alias sc := scraper

