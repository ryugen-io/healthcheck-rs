#!/usr/bin/env bash
set -e

cd "$(dirname "$0")/.."

echo "Stopping and cleaning up benchmark containers..."

# Stop any running containers
docker-compose -f tests/docker-compose.bench.yml down

# Remove volumes (optional - uncomment if you want to clean everything)
# docker-compose -f tests/docker-compose.bench.yml down -v

echo ""
echo "Cleanup complete!"
