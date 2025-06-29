#!/bin/bash
# Common utilities for Mandala-Node maintenance scripts

# Colors for output
export RED='\033[0;31m'
export GREEN='\033[0;32m'
export YELLOW='\033[1;33m'
export BLUE='\033[0;34m'
export NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[*]${NC} $1"
}

print_error() {
    echo -e "${RED}[!]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[!]${NC} $1"
}

print_info() {
    echo -e "${BLUE}[i]${NC} $1"
}

# Check if script is run from project root
check_project_root() {
    if [ ! -d ".maintain" ] || [ ! -d ".maintain/zombienet" ]; then
        print_error "This script must be run from the root of the project."
        return 1
    fi
    return 0
}

# Detect operating system
detect_os() {
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        if grep -q Microsoft /proc/version 2>/dev/null; then
            echo "wsl"
        else
            echo "linux"
        fi
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        echo "macos"
    elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
        echo "windows"
    else
        echo "unknown"
    fi
}

# Detect CPU architecture
detect_arch() {
    local arch=$(uname -m)
    case $arch in
        x86_64)
            echo "x64"
            ;;
        aarch64|arm64)
            echo "arm64"
            ;;
        *)
            echo "$arch"
            ;;
    esac
}

# Get project root directory
get_project_root() {
    git rev-parse --show-toplevel 2>/dev/null || pwd
}

# Check if a command exists
command_exists() {
    command -v "$1" &> /dev/null
}

# Download file with progress
download_file() {
    local url=$1
    local output=$2
    local description=${3:-"file"}
    
    print_status "Downloading $description from:"
    print_status "  $url"
    
    if command_exists curl; then
        curl -L -f -# -o "$output" "$url"
    elif command_exists wget; then
        wget --show-progress -q -O "$output" "$url"
    else
        print_error "Neither curl nor wget found. Please install one of them."
        return 1
    fi
}

# Make file executable
make_executable() {
    local file=$1
    if [ -f "$file" ]; then
        chmod +x "$file"
        return 0
    else
        print_error "File not found: $file"
        return 1
    fi
}

# Export functions and variables so they're available to sourcing scripts
export -f print_status print_error print_warning print_info
export -f check_project_root detect_os detect_arch get_project_root
export -f command_exists download_file make_executable