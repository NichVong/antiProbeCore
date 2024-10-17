#!/bin/bash

diesel setup
diesel migration generate anti_probe
for dir in migrations/*_anti_probe; do
    if [ -d "$dir" ]; then
        cp init.sql "$dir/up.sql"
    fi
done
diesel migration run