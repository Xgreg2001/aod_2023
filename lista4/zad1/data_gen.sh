#!/usr/bin/bash

algorithms=("dinic" "edmonds-karp")

# if data directory does not exist, create it
if [ ! -d "data" ]; then
  mkdir data
fi

# if glpk directory does not exist, create it
if [ ! -d "glpk" ]; then
  mkdir glpk
fi

for (( k = 16; k < 17; k++ )); do
  for algo in "${algorithms[@]}"; do
    for (( i = 0; i < 10; i++ )); do
      echo "Generating data for ${algo} with size ${k} (${i}/10)"
      timeout 3m ./target/release/zad1 --size "$k" --algo "${algo}" --glpk "glpk/${algo}_${k}_${i}.mod" > "data/${algo}_${k}_${i}.txt"
#      return_value=$?
#      if [ $return_value -eq 124 ]; then
#        continue
#      fi
#      glpsol --model "glpk/${algo}_${k}_${i}.mod" > "glpk/${algo}_${k}_${i}.out"
    done
  done
done