#!/usr/bin/env bash
#
# install.sh - Install healthcheck-rs using cargo-auditable
# Checks dependencies and installs with auditable metadata
#
set -euo pipefail

# Colors and Nerd Font Icons
readonly BLUE='\033[0;34m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly RED='\033[0;31m'
readonly NC='\033[0m'

readonly ROCKET=""
readonly CHECK=""
readonly WARN=""
readonly ERROR=""
readonly PACKAGE=""
readonly WRENCH=""
readonly SPARKLE=""
readonly DOWNLOAD=""

# Cleanup on exit
cleanup() {
    local exit_code=$?
    if [ $exit_code -ne 0 ]; then
        echo -e "\n${RED}${ERROR}  Installation failed with exit code ${exit_code}${NC}" >&2
    fi
}
trap cleanup EXIT

# Print header
print_header() {
    echo -e "${BLUE}${ROCKET}  HealthCheck RS Installer${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"
}

# Check for Rust installation
check_rust() {
    echo -e "${BLUE}${PACKAGE}  Checking for Rust installation...${NC}"

    if ! command -v cargo >/dev/null 2>&1; then
        echo -e "${RED}${ERROR}  Rust/Cargo not found!${NC}" >&2
        echo -e "\n${YELLOW}Install Rust from: https://rustup.rs/${NC}" >&2
        echo -e "${BLUE}curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh${NC}\n" >&2
        exit 1
    fi

    echo -e "${GREEN}${CHECK}  Rust found: $(cargo --version)${NC}"
}

# Check and install cargo-auditable
check_cargo_auditable() {
    echo -e "${BLUE}${PACKAGE}  Checking for cargo-auditable...${NC}"

    if ! command -v cargo-auditable >/dev/null 2>&1; then
        echo -e "${YELLOW}${WARN}  cargo-auditable not found, installing...${NC}"
        cargo install cargo-auditable || {
            echo -e "${RED}${ERROR}  Failed to install cargo-auditable${NC}" >&2
            exit 1
        }
        echo -e "${GREEN}${CHECK}  cargo-auditable installed${NC}"
    else
        echo -e "${GREEN}${CHECK}  cargo-auditable found${NC}"
    fi
}

# Install healthcheck binary
install_healthcheck() {
    echo ""
    echo -e "${BLUE}${DOWNLOAD}  Installing healthcheck...${NC}"

    # Check if we're in the repo or installing from git
    if [ -f "health-bin/Cargo.toml" ]; then
        echo -e "${BLUE}${PACKAGE}  Installing from local source...${NC}"
        cargo auditable install --path health-bin --force || {
            echo -e "${RED}${ERROR}  Failed to install from local source${NC}" >&2
            exit 1
        }
    else
        echo -e "${BLUE}${PACKAGE}  Installing from GitHub...${NC}"
        cargo auditable install --git https://github.com/ryugen-io/healthcheck-rs health-bin --force || {
            echo -e "${RED}${ERROR}  Failed to install from GitHub${NC}" >&2
            exit 1
        }
    fi

    echo -e "${GREEN}${CHECK}  healthcheck installed successfully!${NC}"
}

# Verify installation
verify_installation() {
    echo ""
    echo -e "${BLUE}${PACKAGE}  Verifying installation...${NC}"

    if ! command -v healthcheck >/dev/null 2>&1; then
        echo -e "${RED}${ERROR}  healthcheck not found in PATH${NC}" >&2
        exit 1
    fi

    local install_path
    install_path=$(which healthcheck)
    echo -e "${GREEN}${CHECK}  Binary installed at: ${install_path}${NC}"

    # Check for auditable metadata (Linux only)
    if command -v readelf >/dev/null 2>&1; then
        if readelf -p .dep-v0 "$install_path" >/dev/null 2>&1; then
            echo -e "${GREEN}${SPARKLE}  Binary contains auditable dependency metadata${NC}"
        else
            echo -e "${YELLOW}${WARN}  No auditable metadata found${NC}"
        fi
    fi
}

# Show usage information
show_usage() {
    echo ""
    echo -e "${BLUE}${WRENCH}  Usage:${NC}"
    echo -e "  ${GREEN}healthcheck <config-file>${NC}      Run health checks from config"
    echo -e "  ${GREEN}healthcheck generate-conf${NC}      Generate example config file"
    echo -e "  ${GREEN}healthcheck generate-bin${NC}       Generate deployment binary"
    echo -e "  ${GREEN}healthcheck serve${NC}              Start HTTP API server (coming soon)"
    echo -e "  ${GREEN}healthcheck watch${NC}              Watch mode (coming soon)"
    echo ""
    echo -e "${BLUE}${WRENCH}  Example config:${NC}"
    echo -e "  ${YELLOW}tcp:host=localhost,port=8080,timeout_ms=1000${NC}"
    echo -e "  ${YELLOW}http:url=http://localhost:8080/health,timeout_ms=5000${NC}"
    echo -e "  ${YELLOW}database:conn_str=postgresql://user:pass@localhost/db${NC}"
    echo -e "  ${YELLOW}process:name=myapp${NC}"
    echo ""
}

# Main execution
main() {
    print_header
    check_rust
    check_cargo_auditable
    install_healthcheck
    verify_installation
    show_usage

    echo -e "${GREEN}${SPARKLE}  Installation complete!${NC}\n"
}

main "$@"
