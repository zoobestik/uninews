#!/usr/bin/env bash

set -euo pipefail

# Script to build Docker image for multiple CPU platforms
# Usage: ./scripts/docker-image.sh [OPTIONS]
# Options:
#   --platform PLATFORMS  Comma-separated list of platforms (default: linux/amd64,linux/arm64)
#   --push                Push the image to registry after build
#   --tag TAG             Additional tag for the image
#   --help                Show this help message

# Default values
PLATFORMS="${PLATFORMS:-linux/amd64,linux/arm64}"
PUSH_FLAG=""
ADDITIONAL_TAG=""

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --platform)
            PLATFORMS="$2"
            shift 2
            ;;
        --push)
            PUSH_FLAG="--push"
            shift
            ;;
        --tag)
            ADDITIONAL_TAG="$2"
            shift 2
            ;;
        --help)
            echo "Usage: $0 [OPTIONS]"
            echo "Options:"
            echo "  --platform PLATFORMS  Comma-separated list of platforms (default: linux/amd64,linux/arm64)"
            echo "  --push                Push the image to registry after build"
            echo "  --tag TAG             Additional tag for the image"
            echo "  --help                Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Get version from Cargo.toml
BOT_VERSION=$(grep -m1 "^version = " Cargo.toml | cut -d'"' -f2)
IMAGE="zoobestik/uninews"

echo "Building Docker image..."
echo "  Image: $IMAGE"
echo "  Version: $BOT_VERSION"
echo "  Platforms: $PLATFORMS"

# Ensure buildx is available
if ! docker buildx version &> /dev/null; then
    echo "Error: docker buildx is not available"
    echo "Please install Docker Buildx: https://docs.docker.com/buildx/working-with-buildx/"
    exit 1
fi

# Create builder instance if it doesn't exist
if ! docker buildx inspect multiplatform-builder &> /dev/null; then
    echo "Creating buildx builder instance..."
    docker buildx create --name multiplatform-builder --use
fi

# Use the builder
docker buildx use multiplatform-builder

# Build the image
docker buildx build \
    --platform "$PLATFORMS" \
    -t "$IMAGE:$BOT_VERSION" \
    -t "$IMAGE:latest" \
    ${ADDITIONAL_TAG:+-t "$IMAGE:$ADDITIONAL_TAG"} \
    ${PUSH_FLAG} \
    .

echo "Build completed successfully!"
if [[ -n "$PUSH_FLAG" ]]; then
    echo "Image pushed to registry: $IMAGE:$BOT_VERSION"
else
    echo "To push the image, run with --push flag"
fi
