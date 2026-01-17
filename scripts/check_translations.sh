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

find "$root" -type f \( -name '*.md' -o -name '*.rs' \) -not -path "*/$lang/*" | sed "s|^$root/||" | sort > "$ENG_LIST"

if [ ! -d "$root/$lang" ]; then
  echo "Translation directory '$root/$lang' not found" >&2
  echo "You can create translations under $root/<lang>/" >&2
  exit 2
fi

find "$root/$lang" -type f \( -name '*.md' -o -name '*.rs' \) | sed "s|^$root/$lang/||" | sort > "$TRANS_LIST"

eng_count=$(wc -l < "$ENG_LIST" | tr -d ' ')
trans_count=$(wc -l < "$TRANS_LIST" | tr -d ' ')

echo "English files: $eng_count"
echo "$lang files:    $trans_count"

# --- Main Logic ---
exit_code=0

# 1. Calculate median size ratio
total_ratio=0
file_count=0
ratios=""
while read -r file; do
  eng_file="$root/$file"
  trans_file="$root/$lang/$file"
  
  if [ -f "$eng_file" ] && [ -f "$trans_file" ]; then
    eng_size=$(stat -f%z "$eng_file")
    trans_size=$(stat -f%z "$trans_file")
    
    if [ "$eng_size" -gt 0 ]; then
      ratio=$(awk -v trans="$trans_size" -v eng="$eng_size" 'BEGIN { print trans/eng }')
      file_count=$((file_count + 1))
      ratios="$ratios$file $ratio\n"
    fi
  fi
done < <(comm -12 "$ENG_LIST" "$TRANS_LIST")

if [ "$file_count" -gt 0 ]; then
  sorted_ratios=$(echo -e "$ratios" | awk '{print $2}' | sort -n)
  median_ratio=$(echo "$sorted_ratios" | awk -v count="$file_count" '{arr[NR]=$1} END { if (count % 2 == 1) { print arr[int(count/2)+1] } else { print (arr[count/2] + arr[count/2+1]) / 2 } }')
  
  lower_bound=$(awk -v median="$median_ratio" 'BEGIN { print median * 0.80 }')
  upper_bound=$(awk -v median="$median_ratio" 'BEGIN { print median * 1.20 }')

  echo "Median size ratio: $(printf "%.2f" "$median_ratio")"
  echo "Safe range (20% deviation): [$(printf "%.2f" "$lower_bound") - $(printf "%.2f" "$upper_bound")]"
  
  # 2. Check for file size differences
  echo -e "\nFiles with potential translation issues (size outside safe range):"
  
  outliers=$(echo -e "$ratios" | awk -v lower="$lower_bound" -v upper="$upper_bound" -v root="$root" -v lang="$lang" '
    {
      # In awk, $1 is the first field (filename), $2 is the second (ratio)
      filename = $1
      ratio = $2
      
      if (ratio < lower || ratio > upper) {
        trans_file = root "/" lang "/" filename
        
        # The getline command executes the shell command and puts the output into the trans_size variable
        command = "stat -f%z \"" trans_file "\""
        command | getline trans_size
        close(command)

        percentage = ratio * 100
        printf("  - \047%s/%s\047 (Size: %d bytes, %.2f%% of original)\n", lang, filename, trans_size, percentage)
      }
    }
  ')

  if [ -n "$outliers" ]; then
      echo "$outliers" >&2
      exit_code=1
  else
      echo "OK: No files found with significant size differences."
  fi
else
  echo "No files to compare."
fi

# 3. Check for missing or extra files
if [ "$eng_count" -ne "$trans_count" ]; then
  echo -e "\nERROR: File counts do not match for '$lang'." >&2
  exit_code=1
  
  echo -e "\nFiles present in English but missing in $lang:" >&2
  comm -23 "$ENG_LIST" "$TRANS_LIST" || true

  echo -e "\nFiles present in $lang but not in English:" >&2
  comm -13 "$ENG_LIST" "$TRANS_LIST" || true
else
  echo -e "\nOK: File counts match for '$lang'."
fi

exit $exit_code
