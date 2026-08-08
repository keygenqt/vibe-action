#!/bin/bash

# Set root dir to fixtures/
cd "$( dirname -- "${BASH_SOURCE[0]}"; )";

# Setup reports directory and timestamped file
mkdir -p reports
REPORT_FILE="reports/report_$(date +%Y-%m-%d_%H-%M-%S).txt"

# Iterate over all subdirectories and execute their run.sh
for dir in */; do
    if [ -f "${dir}run.sh" ]; then
        echo "--- Fixture: ${dir} ---" | tee -a "$REPORT_FILE"
        bash "${dir}run.sh" 2>&1 | tee -a "$REPORT_FILE"
        echo -e "\n" | tee -a "$REPORT_FILE"
    fi
done

echo "Report saved to: $REPORT_FILE"
