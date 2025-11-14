#!/usr/bin/env bash
#
# lines.sh - Line count analysis for healthcheck-rs
# Analyzes all .rs files with color-coded output
#
set -euo pipefail

# Colors and Nerd Font Icons
readonly BLUE='\033[0;34m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly RED='\033[0;31m'
readonly NC='\033[0m'

readonly CHART=""
readonly FILE=""
readonly WARN=""

# Cleanup on exit
cleanup() {
    local exit_code=$?
    if [ $exit_code -ne 0 ]; then
        echo -e "\n${RED}  Analysis failed with exit code ${exit_code}${NC}" >&2
    fi
}
trap cleanup EXIT

# Check required commands
check_dependencies() {
    local missing=()

    if ! command -v find >/dev/null 2>&1; then
        missing+=("find")
    fi

    if ! command -v wc >/dev/null 2>&1; then
        missing+=("wc")
    fi

    if [ ${#missing[@]} -gt 0 ]; then
        echo -e "${RED}  Missing dependencies: ${missing[*]}${NC}" >&2
        exit 1
    fi
}

# Print header
print_header() {
    echo -e "${BLUE}${CHART}  Line Count Analysis${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"
}

# Analyze files
analyze_files() {
    local rs_files
    rs_files=$(find . -name "*.rs" -not -path "./target/*" | sort)

    if [ -z "$rs_files" ]; then
        echo -e "${YELLOW}${WARN}  No .rs files found${NC}" >&2
        exit 1
    fi

    local total_lines=0
    local total_files=0
    local max_lines=0
    local max_file=""
    local min_lines=999999
    local min_file=""

    echo -e "${BLUE}${FILE}  File Analysis:${NC}"
    echo ""

    # Analyze each file
    while IFS= read -r file; do
        if [ -f "$file" ]; then
            local lines
            lines=$(wc -l < "$file")
            total_lines=$((total_lines + lines))
            total_files=$((total_files + 1))

            # Track max
            if [ $lines -gt $max_lines ]; then
                max_lines=$lines
                max_file=$file
            fi

            # Track min
            if [ $lines -lt $min_lines ]; then
                min_lines=$lines
                min_file=$file
            fi

            # Color code by size
            local color icon
            if [ $lines -gt 100 ]; then
                color=$RED
                icon="${WARN}"
            elif [ $lines -gt 80 ]; then
                color=$YELLOW
                icon="${WARN}"
            else
                color=$GREEN
                icon=" "
            fi

            printf "${color}${icon}  %4d lines${NC}  %s\n" $lines "$file"
        fi
    done <<< "$rs_files"

    # Calculate average
    local avg_lines=0
    if [ $total_files -gt 0 ]; then
        avg_lines=$((total_lines / total_files))
    fi

    echo ""
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${GREEN}Summary:${NC}"
    echo -e "  Total files:   ${total_files}"
    echo -e "  Total lines:   ${total_lines}"
    echo -e "  Average lines: ${avg_lines}"
    echo -e "  Max lines:     ${max_lines} ${YELLOW}(${max_file})${NC}"
    echo -e "  Min lines:     ${min_lines} ${GREEN}(${min_file})${NC}"
    echo ""

    # Check if we have files over 100 lines
    local over_100
    over_100=$(find . -name "*.rs" -not -path "./target/*" -exec wc -l {} \; | awk '$1 > 100 {count++} END {print count+0}')

    if [ $over_100 -gt 0 ]; then
        echo -e "${YELLOW}${WARN}  Warning: ${over_100} file(s) exceed 100 lines${NC}"
    else
        echo -e "${GREEN}  All files under 100 lines!${NC}"
    fi
}

# Main execution
main() {
    print_header
    check_dependencies
    analyze_files
}

main "$@"
