#!/bin/bash

# Navi launchd wrapper script
# Checks if navi has been run in the last 24 hours and runs it if needed

# Configuration
NAVI_DIR="/Users/melvillian/code/navi"
LOG_FILE="$NAVI_DIR/navi_last_run.log"
CARGO_TOML="$NAVI_DIR/Cargo.toml"

# Function to log messages
log_message() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') - $1" >> "$LOG_FILE"
}

# Check if navi directory exists
if [ ! -d "$NAVI_DIR" ]; then
    log_message "ERROR: Navi directory not found at $NAVI_DIR"
    exit 1
fi

# Check if Cargo.toml exists
if [ ! -f "$CARGO_TOML" ]; then
    log_message "ERROR: Cargo.toml not found at $CARGO_TOML"
    exit 1
fi

# Check if log file exists and get last run time
if [ -f "$LOG_FILE" ]; then
    # Get the last line that contains a timestamp (not an error message)
    LAST_RUN_LINE=$(grep "^[0-9]\{4\}-[0-9]\{2\}-[0-9]\{2\} [0-9]\{2\}:[0-9]\{2\}:[0-9]\{2\}" "$LOG_FILE" | tail -1)
    
    if [ -n "$LAST_RUN_LINE" ]; then
        # Extract timestamp from the line
        LAST_RUN_TIME=$(echo "$LAST_RUN_LINE" | cut -d' ' -f1,2)
        
        # Convert to epoch time for comparison
        LAST_RUN_EPOCH=$(date -j -f "%Y-%m-%d %H:%M:%S" "$LAST_RUN_TIME" +%s 2>/dev/null)
        CURRENT_EPOCH=$(date +%s)
        
        # Calculate time difference in hours
        TIME_DIFF_HOURS=$(( (CURRENT_EPOCH - LAST_RUN_EPOCH) / 3600 ))
        
        log_message "Last run was $TIME_DIFF_HOURS hours ago"
        
        # If less than 24 hours, skip execution
        if [ $TIME_DIFF_HOURS -lt 24 ]; then
            log_message "Skipping execution - navi was run recently"
            exit 0
        fi
    fi
else
    log_message "No previous run log found - will execute navi"
fi

# Change to navi directory
cd "$NAVI_DIR" || {
    log_message "ERROR: Failed to change to navi directory"
    exit 1
}

# Run navi
log_message "Starting navi execution"
if cargo run --manifest-path "$CARGO_TOML"; then
    log_message "Navi execution completed successfully"
else
    log_message "ERROR: Navi execution failed"
    exit 1
fi 