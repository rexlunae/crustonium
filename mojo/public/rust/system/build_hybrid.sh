#!/bin/bash
# Quick build script for Mojo Rust System API (Tier 2 component)
# This demonstrates the hybrid build pattern for components with GN-generated bindings
#
# Note: This script is component-specific with hardcoded targets.
# For a general-purpose hybrid build script, see:
# tools/cargo_migration/hybrid_build.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../../.." && pwd)"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

show_help() {
    cat << EOF
Mojo Rust System API Build Script (Tier 2 Hybrid Build)

Usage: $0 [OPTIONS]

Options:
    --check         Just check, don't build
    --test          Run tests after building
    --clean         Clean build artifacts first
    -h, --help      Show this help

Build Process:
    1. Check for/generate GN build files
    2. Build Mojo C API and bindings with Ninja
    3. Build Rust component with Cargo

Examples:
    # Full hybrid build
    $0

    # Check only
    $0 --check

    # Clean and rebuild
    $0 --clean && $0

    # Build and test
    $0 --test
EOF
}

# Parse arguments
DO_CLEAN=false
DO_CHECK=false
DO_TEST=false

while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            show_help
            exit 0
            ;;
        --clean)
            DO_CLEAN=true
            shift
            ;;
        --check)
            DO_CHECK=true
            shift
            ;;
        --test)
            DO_TEST=true
            shift
            ;;
        *)
            echo "Unknown option: $1"
            show_help
            exit 1
            ;;
    esac
done

cd "$REPO_ROOT"

if $DO_CLEAN; then
    log_info "Cleaning build artifacts..."
    rm -rf out/Default/obj/mojo/public/rust
    cargo clean -p mojo-rust-system-api
    log_success "Clean complete"
    exit 0
fi

log_info "=== Mojo Rust System API Hybrid Build ==="
echo ""

# Step 1: Check/Generate GN build files
log_info "Step 1/3: Checking GN build configuration..."
if [ ! -d "out/Default" ]; then
    log_warn "GN build directory not found. Generating..."
    if ! command -v gn &> /dev/null; then
        echo "ERROR: 'gn' command not found"
        echo "Please install GN or run this from a Chromium development environment"
        exit 1
    fi
    gn gen out/Default
    log_success "GN configuration generated"
else
    log_success "GN build directory exists"
fi

# Step 2: Build Mojo C API and bindings
if ! $DO_CHECK; then
    log_info "Step 2/3: Building Mojo C API and bindings..."
    if ! command -v ninja &> /dev/null; then
        echo "ERROR: 'ninja' command not found"
        echo "Please install Ninja or run this from a Chromium development environment"
        exit 1
    fi
    
    # Build Mojo C system bindings
    ninja -C out/Default mojo/public/rust:mojo_c_system_bindings
    log_success "Mojo C API and bindings built"
fi

# Step 3: Build Rust component
log_info "Step 3/3: Building Rust component..."
if $DO_CHECK; then
    cargo check -p mojo-rust-system-api
    log_success "Cargo check passed"
else
    cargo build -p mojo-rust-system-api
    log_success "Rust component built"
fi

echo ""
log_success "Hybrid build complete!"

# Run tests if requested
if $DO_TEST; then
    echo ""
    log_info "Running tests..."
    cargo test -p mojo-rust-system-api
    log_success "Tests passed"
fi

echo ""
log_info "Component: mojo/public/rust/system"
log_info "Build type: Hybrid (GN + Cargo)"
log_info "Status: Tier 2 - GN-Generated Bindings"
