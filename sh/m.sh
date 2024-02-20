#!/bin/bash

# sh/m.sh abc 123

pa="src/"$1"/"$2
echo 🤖_path: ${pa}

mkdir -p ${pa}"/A"
mkdir -p ${pa}"/B"
mkdir -p ${pa}"/C"
mkdir -p ${pa}"/D"

cp src/main.rs ${pa}"/A/main.rs"
cp src/main.rs ${pa}"/B/main.rs"
cp src/main.rs ${pa}"/C/main.rs"
cp src/main.rs ${pa}"/D/main.rs"

echo "💫_complete"
