#!/usr/bin/bash

# create data directory if it doesn't exist
if [ ! -d data ]; then
    mkdir data
fi

cargo build --release

for (( k = 3; k <= 16; k++ )); do
  for (( i = 3; i <= k; i++ )); do
    for (( j = 0; j < 20; j++ )); do
      echo "Generating data for k=$k, i=$i, j=$j"
      ./target/release/zad2 --size "$k" --degree "$i" > "data/$k-$i-$j.txt"
    done
  done
done