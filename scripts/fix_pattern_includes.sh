#!/bin/bash
# Fix include paths in Chinese pattern files to point to English source

set -e

PROJECT_ROOT="/Users/saltuk/code/idiomatic-rust-snippets"

echo "Fixing include paths in Chinese pattern files..."

# For behavioral patterns: need ../../../patterns/behavioral/
find "$PROJECT_ROOT/src/zh-CN/patterns/behavioral" -name "*.md" -type f | while read -r file; do
    if grep -q "{{#include" "$file"; then
        echo "Fixing: $file"
        sed -i '' 's|{{#include \([^/]*\)/src/|{{#include ../../../patterns/behavioral/\1/src/|g' "$file"
    fi
done

# For creational patterns: need ../../../patterns/creational/
find "$PROJECT_ROOT/src/zh-CN/patterns/creational" -name "*.md" -type f | while read -r file; do
    if grep -q "{{#include" "$file"; then
        echo "Fixing: $file"
        sed -i '' 's|{{#include \([^/]*\)/src/|{{#include ../../../patterns/creational/\1/src/|g' "$file"
    fi
done

# For structural patterns: need ../../../patterns/structural/
find "$PROJECT_ROOT/src/zh-CN/patterns/structural" -name "*.md" -type f | while read -r file; do
    if grep -q "{{#include" "$file"; then
        echo "Fixing: $file"
        sed -i '' 's|{{#include \([^/]*\)/src/|{{#include ../../../patterns/structural/\1/src/|g' "$file"
    fi
done

# For rust-idioms patterns: need ../../../patterns/rust-idioms/
find "$PROJECT_ROOT/src/zh-CN/patterns/rust-idioms" -name "*.md" -type f | while read -r file; do
    if grep -q "{{#include" "$file"; then
        echo "Fixing: $file"
        sed -i '' 's|{{#include \([^/]*\)/src/|{{#include ../../../patterns/rust-idioms/\1/src/|g' "$file"
    fi
done

echo "Done! Fixed all include paths."
