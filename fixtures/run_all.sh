#!/bin/bash

# Setup root directory
ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT_DIR"

# Setup reports directory and timestamped report file
mkdir -p reports
REPORT_FILE="reports/report_$(date +%Y-%m-%d_%H-%M-%S).txt"

# Iterate over all group/action directories and execute their run.sh
# tee mirrors output to the terminal (colors preserved) and to the report file (raw)
for dir in fixtures/*/*/; do
    if [ -f "${dir}run.sh" ]; then
        echo "--- Fixture: ${dir} ---" | tee -a "$REPORT_FILE"
        bash "${dir}run.sh" 2>&1 | tee -a "$REPORT_FILE"
        printf '\n\n' | tee -a "$REPORT_FILE"
    fi
done

# Strip ANSI escape codes from the report file for clean LLM ingestion.
# Terminal output above is unaffected - only the file gets cleaned.
ESC=$(printf '\033')  # real ESC byte; "\x1b" doesn't work in BSD sed (macOS)
CR=$(printf '\r')
sed -i.bak \
    -e "s/${ESC}\[[0-9;?]*[a-zA-Z]//g" \
    -e "s/.*${CR}//" \
    "$REPORT_FILE"
rm -f "${REPORT_FILE}.bak"

echo "Report saved and cleaned: $REPORT_FILE"
