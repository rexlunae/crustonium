#!/bin/bash
# Hybrid Build System Wrapper
# Phase 1.2: Tooling Development
#
# This script supports building with both GN/Ninja and Cargo in parallel,
# allowing transparent switching between build systems.

set -e

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
BUILD_CONFIG="${BUILD_CONFIG:-Default}"
BUILD_SYSTEM="${BUILD_SYSTEM:-hybrid}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Help message
show_help() {
    cat << EOF
Hybrid Build System Wrapper - Phase 1.2

Usage: $0 [OPTIONS] [TARGET]

Options:
    -h, --help              Show this help message
    -s, --system <system>   Build system to use (gn|cargo|hybrid) [default: hybrid]
    -c, --config <config>   Build configuration [default: Default]
    -j, --jobs <n>          Number of parallel jobs
    --clean                 Clean build artifacts
    --check                 Check build without compiling
    --test                  Run tests after building
    --bench                 Run benchmarks

Build Systems:
    gn      - Use GN/Ninja (existing build system)
    cargo   - Use Cargo only (for Rust-only builds)
    hybrid  - Use both systems in parallel (recommended)

Environment Variables:
    BUILD_SYSTEM    Override default build system
    BUILD_CONFIG    Build configuration (Default, Release, etc.)
    CARGO_PROFILE   Cargo build profile (dev, release, production)

Examples:
    # Build with hybrid system
    $0

    # Build with Cargo only
    $0 --system cargo

    # Clean and rebuild
    $0 --clean && $0

    # Build and test
    $0 --test

    # Build specific target with GN
    $0 --system gn chrome
EOF
}

# Check if required tools are installed
check_tools() {
    local missing_tools=()
    
    case "$BUILD_SYSTEM" in
        gn|hybrid)
            if ! command -v gn &> /dev/null; then
                missing_tools+=("gn")
            fi
            if ! command -v ninja &> /dev/null; then
                missing_tools+=("ninja")
            fi
            ;;
    esac
    
    case "$BUILD_SYSTEM" in
        cargo|hybrid)
            if ! command -v cargo &> /dev/null; then
                missing_tools+=("cargo")
            fi
            ;;
    esac
    
    if [ ${#missing_tools[@]} -gt 0 ]; then
        log_error "Missing required tools: ${missing_tools[*]}"
        log_info "Please install missing tools and try again"
        return 1
    fi
    
    return 0
}

# Build with GN/Ninja
build_gn() {
    local target="${1:-chrome}"
    local out_dir="out/$BUILD_CONFIG"
    
    log_info "Building with GN/Ninja..."
    log_info "Configuration: $BUILD_CONFIG"
    log_info "Target: $target"
    
    # Generate build files if needed
    if [ ! -d "$REPO_ROOT/$out_dir" ]; then
        log_info "Generating build files..."
        cd "$REPO_ROOT"
        gn gen "$out_dir" --args="use_cargo_rust=true"
    fi
    
    # Build with Ninja
    log_info "Building with Ninja..."
    cd "$REPO_ROOT"
    
    local ninja_args=()
    [ -n "$JOBS" ] && ninja_args+=("-j$JOBS")
    
    ninja -C "$out_dir" "${ninja_args[@]}" "$target"
    
    log_success "GN/Ninja build complete"
}

# Build with Cargo
build_cargo() {
    local profile="${CARGO_PROFILE:-dev}"
    
    log_info "Building with Cargo..."
    log_info "Profile: $profile"
    
    cd "$REPO_ROOT"
    
    local cargo_args=("build" "--workspace")
    
    case "$profile" in
        release|production)
            cargo_args+=("--release")
            ;;
        dev)
            # Default, no flag needed
            ;;
    esac
    
    [ -n "$JOBS" ] && cargo_args+=("-j$JOBS")
    
    cargo "${cargo_args[@]}"
    
    log_success "Cargo build complete"
}

# Build with hybrid system
build_hybrid() {
    local target="${1:-chrome}"
    
    log_info "Building with hybrid system..."
    log_info "This will build Rust components with Cargo and C++ with GN/Ninja"
    
    # Step 1: Build Rust components with Cargo
    log_info "Step 1/2: Building Rust components..."
    build_cargo
    
    # Step 2: Build C++ and link with Rust
    log_info "Step 2/2: Building C++ components..."
    build_gn "$target"
    
    log_success "Hybrid build complete"
}

# Clean build artifacts
clean_build() {
    log_info "Cleaning build artifacts..."
    
    case "$BUILD_SYSTEM" in
        gn|hybrid)
            if [ -d "$REPO_ROOT/out" ]; then
                log_info "Removing out/ directory..."
                rm -rf "$REPO_ROOT/out"
            fi
            ;;
    esac
    
    case "$BUILD_SYSTEM" in
        cargo|hybrid)
            if [ -d "$REPO_ROOT/target" ]; then
                log_info "Removing target/ directory..."
                cargo clean
            fi
            ;;
    esac
    
    log_success "Clean complete"
}

# Check build without compiling
check_build() {
    log_info "Checking build configuration..."
    
    case "$BUILD_SYSTEM" in
        cargo|hybrid)
            cd "$REPO_ROOT"
            cargo check --workspace
            log_success "Cargo check passed"
            ;;
    esac
    
    case "$BUILD_SYSTEM" in
        gn)
            log_info "GN check not implemented (build would validate)"
            ;;
    esac
}

# Run tests
run_tests() {
    log_info "Running tests..."
    
    case "$BUILD_SYSTEM" in
        cargo|hybrid)
            cd "$REPO_ROOT"
            cargo test --workspace
            log_success "Cargo tests passed"
            ;;
    esac
    
    case "$BUILD_SYSTEM" in
        gn|hybrid)
            local out_dir="out/$BUILD_CONFIG"
            if [ -f "$REPO_ROOT/$out_dir/unit_tests" ]; then
                "$REPO_ROOT/$out_dir/unit_tests"
                log_success "GN tests passed"
            else
                log_warn "GN tests not found (build first)"
            fi
            ;;
    esac
}

# Run benchmarks
run_benchmarks() {
    log_info "Running benchmarks..."
    
    case "$BUILD_SYSTEM" in
        cargo|hybrid)
            cd "$REPO_ROOT"
            cargo bench --workspace
            log_success "Benchmarks complete"
            ;;
    esac
}

# Main function
main() {
    local target=""
    local do_clean=false
    local do_check=false
    local do_test=false
    local do_bench=false
    
    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help)
                show_help
                exit 0
                ;;
            -s|--system)
                BUILD_SYSTEM="$2"
                shift 2
                ;;
            -c|--config)
                BUILD_CONFIG="$2"
                shift 2
                ;;
            -j|--jobs)
                JOBS="$2"
                shift 2
                ;;
            --clean)
                do_clean=true
                shift
                ;;
            --check)
                do_check=true
                shift
                ;;
            --test)
                do_test=true
                shift
                ;;
            --bench)
                do_bench=true
                shift
                ;;
            *)
                target="$1"
                shift
                ;;
        esac
    done
    
    # Validate build system
    case "$BUILD_SYSTEM" in
        gn|cargo|hybrid)
            ;;
        *)
            log_error "Invalid build system: $BUILD_SYSTEM"
            log_info "Valid options: gn, cargo, hybrid"
            exit 1
            ;;
    esac
    
    # Check tools
    if ! check_tools; then
        exit 1
    fi
    
    # Execute requested operations
    if $do_clean; then
        clean_build
        exit 0
    fi
    
    if $do_check; then
        check_build
        exit 0
    fi
    
    # Build
    log_info "=== Chromium/Crustonium Hybrid Build ==="
    log_info "Build System: $BUILD_SYSTEM"
    log_info "Configuration: $BUILD_CONFIG"
    echo
    
    case "$BUILD_SYSTEM" in
        gn)
            build_gn "$target"
            ;;
        cargo)
            build_cargo
            ;;
        hybrid)
            build_hybrid "$target"
            ;;
    esac
    
    # Post-build operations
    if $do_test; then
        echo
        run_tests
    fi
    
    if $do_bench; then
        echo
        run_benchmarks
    fi
    
    echo
    log_success "All operations complete!"
}

# Run main function
main "$@"
