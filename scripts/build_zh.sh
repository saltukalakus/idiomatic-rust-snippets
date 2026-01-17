#!/bin/bash
# Build Chinese translation of the book

set -e

# Get script directory and project root
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$( cd "$SCRIPT_DIR/.." && pwd )"

echo "Building Chinese translation..."

# Create temp directory for book config
TEMP_CONFIG=$(mktemp)
trap "rm -f $TEMP_CONFIG" EXIT

# Create temporary config that points to Chinese sources
cat > "$TEMP_CONFIG" << 'EOF'
[book]
authors = ["saltukalakus"]
language = "zh-CN"
multilingual = false
src = "src/zh-CN"
title = "惯用 Rust 代码片段"

[output.html.playpen]
editable = true
editor = "ace"
line-numbers = false

[output.html.fold]
enable = true

[output.html]
git-repository-url = "https://github.com/saltukalakus/idiomatic-rust-snippets"
git-repository-icon = "fa-github"
edit-url-template = "https://github.com/saltukalakus/idiomatic-rust-snippets/edit/main/{path}"
additional-css = ["custom.css"]

[output.html.print]
enable = false

[preprocessor.metadata]
valid-tags = ["title", "author", "keywords", "released"]
default-author = "Jane Doe"
continue-on-error = true

[build]
build-dir = "book/zh-CN"
EOF

# Backup original config
cp "$PROJECT_ROOT/book.toml" "$PROJECT_ROOT/book.toml.backup"

# Use temporary config
cp "$TEMP_CONFIG" "$PROJECT_ROOT/book.toml"

# Build
cd "$PROJECT_ROOT"
mdbook build

# Restore original config
mv -f "$PROJECT_ROOT/book.toml.backup" "$PROJECT_ROOT/book.toml"
rm -f "$TEMP_CONFIG"

echo "Chinese translation built successfully!"
echo "$(find "$PROJECT_ROOT/book/zh-CN" -name "*.html" 2>/dev/null | wc -l | tr -d ' ') HTML files generated"
