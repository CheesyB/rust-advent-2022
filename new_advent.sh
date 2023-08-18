
set -Eeuo pipefail

target_dir=./src

if [ ! -d "$target_dir" ]; then
    echo "Directory not found: $target_dir"
    exit 1
fi

foldername=advent$(($(find "$target_dir" -type d -name "advent*" | wc -l ) +1))

mkdir $target_dir/$foldername
touch $target_dir/$foldername/mod.rs
touch $target_dir/$foldername/$1