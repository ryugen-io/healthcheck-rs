#!/usr/bin/env bash
#
# rebuild.sh - Complete rebuild pipeline for healthcheck-rs
# Runs: cargo fmt → cargo clippy → cargo auditable build
#
set -euo pipefail

# Colors and Nerd Font Icons
readonly BLUE='\033[0;34m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly RED='\033[0;31m'
readonly NC='\033[0m'

readonly CHECK=""
readonly ROCKET=""
readonly WRENCH=""
readonly WARN=""
readonly ERROR=""

# Cleanup on exit
cleanup() {
    local exit_code=$?
    if [ $exit_code -ne 0 ]; then
        echo -e "\n${RED}${ERROR}  Build failed with exit code ${exit_code}${NC}" >&2
    fi
}
trap cleanup EXIT

# Check required commands
check_dependencies() {
    local missing=()

    if ! command -v cargo >/dev/null 2>&1; then
        missing+=("cargo")
    fi

    if ! command -v cargo-auditable >/dev/null 2>&1; then
        echo -e "${YELLOW}${WARN}  cargo-auditable not found, installing...${NC}"
        cargo install cargo-auditable || {
            echo -e "${RED}${ERROR}  Failed to install cargo-auditable${NC}" >&2
            exit 1
        }
    fi

    if [ ${#missing[@]} -gt 0 ]; then
        echo -e "${RED}${ERROR}  Missing dependencies: ${missing[*]}${NC}" >&2
        echo "Install from: https://rustup.rs/" >&2
        exit 1
    fi
}

# Print header
print_header() {
    echo -e "${BLUE}${WRENCH}  HealthCheck RS - Rebuild${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"
}

# Run cargo fmt
run_fmt() {
    echo -e "${BLUE}[1/3] Running cargo fmt...${NC}"
    if cargo fmt --all -- --check >/dev/null 2>&1; then
        echo -e "${GREEN}${CHECK}  Code already formatted${NC}\n"
    else
        cargo fmt --all
        echo -e "${GREEN}${CHECK}  Formatting complete${NC}\n"
    fi
}

# Run cargo clippy
run_clippy() {
    echo -e "${BLUE}[2/3] Running cargo clippy...${NC}"
    if cargo clippy --all-targets --all-features -- -D warnings 2>&1; then
        echo -e "${GREEN}${CHECK}  Clippy passed${NC}\n"
    else
        echo -e "${RED}${ERROR}  Clippy found issues${NC}" >&2
        return 1
    fi
}

# Run cargo auditable build
run_build() {
    echo -e "${BLUE}[3/3] Running cargo auditable build...${NC}"
    cargo auditable build --release
    echo -e "${GREEN}${CHECK}  Build successful${NC}\n"
}

# Show binary info
show_binary_info() {
    local binary="target/release/healthcheck"

    if [ ! -f "$binary" ]; then
        echo -e "${YELLOW}${WARN}  Binary not found at $binary${NC}" >&2
        return 1
    fi

    local size
    size=$(ls -lh "$binary" 2>/dev/null | awk '{print $5}')
    echo -e "${GREEN}${ROCKET}  Binary ready: ${binary} (${size})${NC}"

    # Check for auditable metadata
    if command -v readelf >/dev/null 2>&1; then
        if readelf -p .dep-v0 "$binary" >/dev/null 2>&1; then
            echo -e "${GREEN}${CHECK}  Auditable metadata present${NC}"
        else
            echo -e "${YELLOW}${WARN}  No auditable metadata found${NC}"
        fi
    fi
}

# Main execution
main() {
    print_header
    check_dependencies
    run_fmt
    run_clippy
    run_build
    show_binary_info

    echo -e "\n${GREEN}${CHECK}  All checks passed!${NC}"
}

main "$@"
