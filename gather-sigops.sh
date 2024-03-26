#!/usr/bin/env bash

set -e

HOST="http://127.0.0.1:8332"
START=760000
END=840000

for height in $(seq $START $END);
do
  hash=$(curl -s "$HOST/rest/blockhashbyheight/$height.hex")
  block=$(curl -s "$HOST/rest/block/$hash.hex")
  sigops=$(echo $block | ./target/release/stdin-count-block-sigops)
  echo "$height,$hash,$sigops"
done
