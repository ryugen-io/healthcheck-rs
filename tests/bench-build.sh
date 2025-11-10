#!/usr/bin/env bash
set -e

cd "$(dirname "$0")/.."

echo "Building benchmark container..."
docker-compose -f tests/docker-compose.bench.yml build

echo ""
echo "Build complete. Run './tests/bench-run.sh' to execute benchmarks."
