#!/usr/bin/env bash
set -e

# Colors and icons
BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

CHECK=""
ROCKET=""
WRENCH=""
WARN=""

echo -e "${BLUE}${WRENCH}  HealthCheck RS - Rebuild${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"

# Step 1: Format
echo -e "${BLUE}1️⃣  Running cargo fmt...${NC}"
cargo fmt --all
echo -e "${GREEN}${CHECK}  Formatting complete${NC}\n"

# Step 2: Clippy
echo -e "${BLUE}2️⃣  Running cargo clippy...${NC}"
if cargo clippy --all-targets --all-features -- -D warnings; then
    echo -e "${GREEN}${CHECK}  Clippy passed${NC}\n"
else
    echo -e "${RED}✗  Clippy failed${NC}"
    exit 1
fi

# Step 3: Build
echo -e "${BLUE}3️⃣  Running cargo auditable build...${NC}"
if cargo auditable build --release; then
    echo -e "${GREEN}${CHECK}  Build successful${NC}\n"
else
    echo -e "${RED}✗  Build failed${NC}"
    exit 1
fi

# Step 4: Show binary info
BINARY="target/release/healthcheck"
if [ -f "$BINARY" ]; then
    SIZE=$(ls -lh "$BINARY" | awk '{print $5}')
    echo -e "${GREEN}${ROCKET}  Binary ready: ${BINARY} (${SIZE})${NC}"

    # Check for auditable metadata
    if readelf -p .dep-v0 "$BINARY" &> /dev/null; then
        echo -e "${GREEN}${CHECK}  Auditable metadata present${NC}"
    else
        echo -e "${YELLOW}${WARN}  No auditable metadata found${NC}"
    fi
fi

echo -e "\n${GREEN}${CHECK}  All checks passed!${NC}"
