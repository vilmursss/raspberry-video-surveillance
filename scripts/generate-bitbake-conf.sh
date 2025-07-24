#!/bin/bash
set -e

INPUT_FILE="$1"
OUTPUT_FILE="$2"

if [ ! -f "$INPUT_FILE" ]; then
  echo "Input file $INPUT_FILE not found!"
  exit 1
fi

echo "# Auto-generated from $INPUT_FILE on $(date)" > "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

while IFS= read -r line; do
  # Strip comments and whitespace
  clean_line="$(echo "$line" | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')"

  # Skip empty lines or preserve comments as-is
  if [ -z "$clean_line" ] || [[ "$clean_line" == \#* ]]; then
    echo "$line" >> "$OUTPUT_FILE"
    continue
  fi

  # Match variable assignments like VAR="value" or VAR=value
  if [[ "$clean_line" =~ ^([A-Za-z_][A-Za-z0-9_]*)=\"(.*)\"$ ]]; then
    # Quoted values: VAR="value"
    var="${BASH_REMATCH[1]}"
    val="${BASH_REMATCH[2]}"
    echo "${var} = \"${val}\"" >> "$OUTPUT_FILE"
  elif [[ "$clean_line" =~ ^([A-Za-z_][A-Za-z0-9_]*)=(.*)$ ]]; then
    # Unquoted values: VAR=value
    var="${BASH_REMATCH[1]}"
    val="${BASH_REMATCH[2]}"
    echo "${var} = \"${val}\"" >> "$OUTPUT_FILE"
  else
    # Skip unsupported non-comment lines
    echo "# Skipped unsupported line: $line" >> "$OUTPUT_FILE"
  fi
done < "$INPUT_FILE"