#!/bin/bash

set -euo pipefail

if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <day>"
    echo "Example: $0 1"
    echo "Example: $0 09"
    exit 1
fi

# Convert to integer (removes leading zeros) then to zero-padded
# Use 10# prefix to force base-10 interpretation (avoids octal issues with 08, 09)
DAY=$((10#$1))
DAY_WITH_LEADING_ZEROES=$(printf "%02d" "$DAY")

# Create directory and copy template
mkdir -p "day$DAY_WITH_LEADING_ZEROES"
cp -Rfv day-template/* "day$DAY_WITH_LEADING_ZEROES"

# Replace placeholders in the copied files
find "day$DAY_WITH_LEADING_ZEROES" -type f \( -name "*.rs" -o -name "*.toml" \) | while read -r file; do
    sed -i "s/__DAY_WITH_LEADING_ZEROES__/$DAY_WITH_LEADING_ZEROES/g" "$file"
    sed -i "s/__DAY__/$DAY/g" "$file"
done

echo "Creating input/$DAY_WITH_LEADING_ZEROES.txt"
mkdir -p input
touch "input/$DAY_WITH_LEADING_ZEROES.txt"

# Add to workspace Cargo.toml if not already present
if ! grep -q "'day$DAY_WITH_LEADING_ZEROES'" Cargo.toml; then
    echo "Adding day$DAY_WITH_LEADING_ZEROES to workspace"
    sed -i "/members = \[/a\\   'day$DAY_WITH_LEADING_ZEROES'," Cargo.toml
fi
