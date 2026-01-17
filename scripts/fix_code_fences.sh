#!/bin/bash
# Fix markdown files wrapped in extra code fences

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$( cd "$SCRIPT_DIR/.." && pwd )"

echo "Fixing markdown code fences in Chinese translations..."

# Find all .md files with ````markdown at the start
find "$PROJECT_ROOT/src/zh-CN" -name "*.md" -type f | while read -r file; do
    # Check if file starts with ````markdown
    if head -n 1 "$file" | grep -q '^````markdown$'; then
        echo "Fixing: $file"
        
        # Create temp file
        temp_file=$(mktemp)
        
        # Remove first line and last line
        # Use sed for macOS compatibility
        sed '1d;$d' "$file" > "$temp_file"
        
        # Replace original file
        mv "$temp_file" "$file"
    fi
done

echo "Done! Fixed markdown code fences."
