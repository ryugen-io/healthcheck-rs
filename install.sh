#!/usr/bin/env bash
set -e

# Colors and Nerd Font icons
BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

ROCKET=""
CHECK=""
WARN=""
ERROR=""
PACKAGE=""
WRENCH=""
SPARKLE=""
DOWNLOAD=""

print_header() {
    echo -e "${BLUE}${ROCKET}  HealthCheck RS Installer${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"
}

print_success() {
    echo -e "${GREEN}${CHECK}  $1${NC}"
}

print_info() {
    echo -e "${BLUE}${PACKAGE}  $1${NC}"
}

print_warn() {
    echo -e "${YELLOW}${WARN}  $1${NC}"
}

print_error() {
    echo -e "${RED}${ERROR}  $1${NC}"
}

check_rust() {
    print_info "Checking for Rust installation..."
    if ! command -v cargo &> /dev/null; then
        print_error "Rust/Cargo not found!"
        echo -e "\n${YELLOW}Please install Rust from:${NC} https://rustup.rs/"
        echo -e "Run: ${BLUE}curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh${NC}\n"
        exit 1
    fi
    print_success "Rust found: $(cargo --version)"
}

check_cargo_auditable() {
    print_info "Checking for cargo-auditable..."
    if ! command -v cargo-auditable &> /dev/null; then
        print_warn "cargo-auditable not found, installing..."
        cargo install cargo-auditable
        print_success "cargo-auditable installed"
    else
        print_success "cargo-auditable found"
    fi
}

install_healthcheck() {
    print_info "${DOWNLOAD}  Installing healthcheck..."

    # Check if we're in the repo or installing from git
    if [ -f "health-bin/Cargo.toml" ]; then
        print_info "Installing from local source..."
        cargo auditable install --path health-bin --force
    else
        print_info "Installing from GitHub..."
        cargo auditable install --git https://github.com/ryugen-io/healthcheck-rs healthcheck-bin --force
    fi

    print_success "healthcheck installed successfully!"
}

verify_installation() {
    print_info "Verifying installation..."

    if command -v healthcheck &> /dev/null; then
        INSTALL_PATH=$(which healthcheck)
        print_success "Binary installed at: ${INSTALL_PATH}"

        # Check for auditable metadata
        if readelf -p .dep-v0 "$INSTALL_PATH" &> /dev/null; then
            print_success "${SPARKLE}  Binary contains auditable dependency metadata"
        else
            print_warn "No auditable metadata found (this is normal for non-auditable builds)"
        fi
    else
        print_error "healthcheck not found in PATH"
        exit 1
    fi
}

show_usage() {
    echo -e "\n${BLUE}${WRENCH}  Usage:${NC}"
    echo -e "  ${GREEN}healthcheck <config-file>${NC}      Run health checks from config"
    echo -e "  ${GREEN}healthcheck serve${NC}              Start HTTP API server (coming soon)"
    echo -e "  ${GREEN}healthcheck watch${NC}              Watch mode (coming soon)"
    echo -e ""
    echo -e "${BLUE}${WRENCH}  Example config:${NC}"
    echo -e "  ${YELLOW}tcp:host=localhost,port=8080,timeout_ms=1000${NC}"
    echo -e "  ${YELLOW}http:url=http://localhost:8080/health,timeout_ms=5000${NC}"
    echo -e "  ${YELLOW}database:conn_str=postgresql://user:pass@localhost/db${NC}"
    echo -e "  ${YELLOW}process:name=myapp${NC}"
    echo -e ""
}

main() {
    print_header
    check_rust
    check_cargo_auditable
    echo ""
    install_healthcheck
    echo ""
    verify_installation
    show_usage

    echo -e "${GREEN}${SPARKLE}  Installation complete!${NC}\n"
}

main "$@"
