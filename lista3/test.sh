#! /usr/bin/bash

# inputs folder
input_folder="ch9-1.1/inputs"

# outputs folder
output_folder="outputs"

# create outputs folder if it doesn't exist
if [ ! -d "$output_folder" ]; then
    mkdir $output_folder
fi

#build the project
make

# for every subfolder in inputs
for folder in $input_folder/*; do
    # for every file in subfolder that ends with .ss
    for file in $folder/*.ss; do
        # run the file
        echo "Running $file"
        # get the filename without the extension
        filename=$(basename -- "$file")
        filename="${filename%.*}"

        # run program on file and save the output to a file with timeout of 1 minute
        timeout 3m ./dial -d "$folder/$filename.gr" -ss "$file" -oss "$output_folder/dial_$filename.ss.res"
        timeout 3m ./dijkstra -d "$folder/$filename.gr" -ss "$file" -oss "$output_folder/dijkstra_$filename.ss.res"
    done
done