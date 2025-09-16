#!/usr/bin/env bash
cargo sqlx prepare --check
ret=$?
if [ 0 -ne $ret ]; then
      echo 'Please run cargo sqlx prepare and commit the updated .sqlx/*json files so the project can be built without database'
      exit $ret
fi



exit $ret
