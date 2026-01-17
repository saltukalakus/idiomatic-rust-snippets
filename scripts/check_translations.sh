#!/usr/bin/env bash
set -euo pipefail

# Usage: ./scripts/check_translations.sh [lang] [root]
# Default: lang=zh-CN, root resolves to the repository's `src` directory

lang=${1:-zh-CN}
# Resolve script directory and default root to repo-root/src so the script
# works when executed from any CWD.
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root=${2:-"$script_dir/../src"}

ENG_LIST=$(mktemp)
TRANS_LIST=$(mktemp)
trap 'rm -f "${ENG_LIST}" "${TRANS_LIST}"' EXIT

if [ ! -d "$root" ]; then
  echo "Root directory '$root' not found" >&2
  exit 2
fi

find "$root" -type f -name '*.md' -not -path "*/$lang/*" | sed "s|^$root/||" | sort > "$ENG_LIST"

if [ ! -d "$root/$lang" ]; then
  echo "Translation directory '$root/$lang' not found" >&2
  echo "You can create translations under $root/<lang>/" >&2
  exit 2
fi

find "$root/$lang" -type f -name '*.md' | sed "s|^$root/$lang/||" | sort > "$TRANS_LIST"

eng_count=$(wc -l < "$ENG_LIST" | tr -d ' ')
trans_count=$(wc -l < "$TRANS_LIST" | tr -d ' ')

echo "English files: $eng_count"
echo "$lang files:    $trans_count"

if [ "$eng_count" -eq "$trans_count" ]; then
  echo "OK: file counts match for '$lang'"
  exit 0
fi

echo "\nFiles present in English but missing in $lang:" >&2
comm -23 "$ENG_LIST" "$TRANS_LIST" || true

echo "\nFiles present in $lang but not in English:" >&2
comm -13 "$ENG_LIST" "$TRANS_LIST" || true

exit 1
