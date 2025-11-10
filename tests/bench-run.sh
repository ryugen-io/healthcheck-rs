#!/usr/bin/env bash
set -e

cd "$(dirname "$0")/.."

echo "Running comprehensive benchmarks..."
echo ""

docker-compose -f tests/docker-compose.bench.yml run --rm healthcheck-bench /usr/local/bin/run-bench.sh

echo ""
echo "Benchmarks complete!"
echo ""
echo "To extract criterion HTML reports:"
echo "  docker cp healthcheck-bench:/workspace/target/criterion ./criterion-reports"
