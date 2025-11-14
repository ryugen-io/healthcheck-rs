#!/usr/bin/env bash
# Ultra-fast UPX compression script for healthcheck binaries
# Builds for multiple platforms and compresses with UPX --best
#
# Usage:
#   ./compress.sh          # Build and compress all targets
#   ./compress.sh --native # Build and compress only native target (faster)

set -euo pipefail

# Colors for output
readonly RED='\033[0;31m'
readonly GREEN='\033[0;32m'
readonly BLUE='\033[0;34m'
readonly YELLOW='\033[1;33m'
readonly NC='\033[0m' # No Color

# Nerd font icons
readonly ROCKET=$'\xee\x82\xaf'  #
readonly CHECK=$'\xee\x83\xbc'   #
readonly ERROR=$'\xee\x84\x80'   #
readonly PACKAGE=$'\xee\x85\x89' #
readonly INFO=$'\xee\x85\xa2'    #

echo -e "${BLUE}  UPX Compression - HealthCheck RS${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"

# Check if UPX is installed
if ! command -v upx >/dev/null 2>&1; then
    echo -e "${ERROR} ${RED}Error: UPX not found${NC}" >&2
    echo -e "${INFO} Install with: sudo apt install upx-ucl  # Debian/Ubuntu" >&2
    echo -e "${INFO}         or: brew install upx            # macOS" >&2
    echo -e "${INFO}         or: pacman -S upx              # Arch Linux" >&2
    exit 1
fi

# Parse command line arguments
NATIVE_ONLY=false
if [[ "${1:-}" == "--native" ]]; then
    NATIVE_ONLY=true
fi

# Detect native target
NATIVE_TARGET=$(rustc -vV | grep host | awk '{print $2}')

# Define targets
# Only include targets that UPX supports (Linux, Windows, some macOS)
if [ "$NATIVE_ONLY" = true ]; then
    readonly TARGETS=("$NATIVE_TARGET")
    echo -e "${INFO} Native-only mode: building for $NATIVE_TARGET\n"
else
    readonly TARGETS=(
        "x86_64-unknown-linux-musl"   # Linux x86_64 (static, musl libc)
        "aarch64-unknown-linux-musl"  # Linux ARM64 (Raspberry Pi, etc.)
        "x86_64-pc-windows-gnu"       # Windows x86_64 (MinGW)
    )
fi

# Function to get human-readable size
get_size() {
    local file=$1
    if [ ! -f "$file" ]; then
        echo "N/A"
        return
    fi

    if command -v numfmt >/dev/null 2>&1; then
        numfmt --to=iec-i --suffix=B < <(stat -c%s "$file" 2>/dev/null || stat -f%z "$file" 2>/dev/null)
    else
        ls -lh "$file" 2>/dev/null | awk '{print $5}'
    fi
}

# Function to compress a single binary
compress_binary() {
    local binary=$1
    local target=$2

    if [ ! -f "$binary" ]; then
        echo -e "${YELLOW}  Skipping $target (binary not found)${NC}"
        return
    fi

    local size_before
    local size_after
    local size_before_bytes
    local size_after_bytes

    size_before=$(get_size "$binary")
    size_before_bytes=$(stat -c%s "$binary" 2>/dev/null || stat -f%z "$binary" 2>/dev/null)

    echo -e "${BLUE}${PACKAGE}  Compressing: ${target}${NC}"
    echo -e "   Before: ${size_before}"

    # Compress with UPX --best (maximum compression)
    # --lzma for better compression on supported platforms
    if upx --best --lzma "$binary" >/dev/null 2>&1; then
        :
    elif upx --best "$binary" >/dev/null 2>&1; then
        :
    else
        echo -e "${ERROR} ${RED}  Failed to compress${NC}"
        return
    fi

    size_after=$(get_size "$binary")
    size_after_bytes=$(stat -c%s "$binary" 2>/dev/null || stat -f%z "$binary" 2>/dev/null)

    # Calculate compression ratio
    local ratio
    ratio=$(awk "BEGIN {printf \"%.1f\", ($size_before_bytes - $size_after_bytes) / $size_before_bytes * 100}")

    echo -e "   After:  ${size_after} ${GREEN}(-${ratio}%)${NC}\n"
}

# Main build and compression loop
total_targets=${#TARGETS[@]}
for i in "${!TARGETS[@]}"; do
    target="${TARGETS[$i]}"
    progress=$((i + 1))

    echo -e "${BLUE}[$progress/$total_targets] Building: ${target}${NC}"

    # Check if target is installed
    if ! rustup target list --installed | grep -q "$target"; then
        echo -e "${INFO} Installing target: ${target}"
        rustup target add "$target"
    fi

    # Build release binary
    if ! cargo build --release --target "$target" >/dev/null 2>&1; then
        echo -e "${ERROR} ${RED}Build failed for $target${NC}"
        echo -e "${INFO} This target may require additional toolchains${NC}\n"
        continue
    fi

    echo -e "${CHECK} ${GREEN}Build successful${NC}\n"

    # Determine binary path and name
    if [[ "$target" == *"windows"* ]]; then
        binary="target/$target/release/healthcheck.exe"
    else
        binary="target/$target/release/healthcheck"
    fi

    # Compress the binary
    compress_binary "$binary" "$target"
done

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}${CHECK}  Compression complete!${NC}\n"

# Summary table
echo -e "${BLUE}${INFO}  Binary Size Summary:${NC}\n"
printf "%-35s %15s\n" "Target" "Compressed Size"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

for target in "${TARGETS[@]}"; do
    if [[ "$target" == *"windows"* ]]; then
        binary="target/$target/release/healthcheck.exe"
    else
        binary="target/$target/release/healthcheck"
    fi

    if [ -f "$binary" ]; then
        size=$(get_size "$binary")
        printf "%-35s %15s\n" "$target" "$size"
    else
        printf "%-35s %15s\n" "$target" "Not built"
    fi
done

echo ""
