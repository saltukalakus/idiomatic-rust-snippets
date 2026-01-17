#!/bin/bash
# Sync include directives from English pattern files to Chinese translations

set -e

PROJECT_ROOT="/Users/saltuk/code/idiomatic-rust-snippets"

echo "Syncing pattern include directives from English to Chinese..."

# List of pattern files that use includes
PATTERN_FILES=(
    "rust-idioms/extension-traits.md"
    "rust-idioms/typestate.md"
    "rust-idioms/newtype.md"
    "rust-idioms/raii.md"
    "creational/abstract-method.md"
    "creational/builder.md"
    "creational/factory-method.md"
    "creational/lazy-initialization.md"
    "behavioral/message-passing.md"
    "behavioral/strategy.md"
    "behavioral/enum-polymorphism.md"
    "behavioral/chain-of-responsibility.md"
    "behavioral/mediator.md"
    "structural/builder-pattern.md"
    "structural/facade.md"
    "structural/bridge.md"
    "structural/adapter.md"
    "structural/proxy.md"
    "structural/flyweight.md"
)

for file in "${PATTERN_FILES[@]}"; do
    EN_FILE="$PROJECT_ROOT/src/patterns/$file"
    ZH_FILE="$PROJECT_ROOT/src/zh-CN/patterns/$file"
    
    if [[ -f "$EN_FILE" ]]; then
        echo "Processing: $file"
        
        # Copy English file to Chinese (it will be translated later, but at least includes will work)
        cp "$EN_FILE" "$ZH_FILE"
    fi
done

echo "Done! Synced ${#PATTERN_FILES[@]} pattern files."
echo "Note: These files now have English text and need manual translation."
