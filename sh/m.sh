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

echo [[bin]] >> Cargo.toml
echo name = \"$1"-"$2"-a\"" >> Cargo.toml
echo "path = \"src/$1/$2/A/main.rs\"" >> Cargo.toml
echo [[bin]] >> Cargo.toml
echo name = \"$1"-"$2"-b\"" >> Cargo.toml
echo "path = \"src/$1/$2/B/main.rs\"" >> Cargo.toml
echo [[bin]] >> Cargo.toml
echo name = \"$1"-"$2"-c\"" >> Cargo.toml
echo "path = \"src/$1/$2/C/main.rs\"" >> Cargo.toml
echo [[bin]] >> Cargo.toml
echo name = \"$1"-"$2"-d\"" >> Cargo.toml
echo "path = \"src/$1/$2/D/main.rs\"" >> Cargo.toml

echo "💫_complete"
