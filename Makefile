# Project name - change this to your binary name
BINARY_NAME := glue

# Output directories
BUILD_DIR := build
RELEASE_DIR := release

# Target triples
LINUX_TARGET := x86_64-unknown-linux-gnu
WINDOWS_TARGET := x86_64-pc-windows-msvc
MACOS_TARGET := x86_64-apple-darwin

# Binary names with platform suffixes
LINUX_BINARY := $(BINARY_NAME)-linux
WINDOWS_BINARY := $(BINARY_NAME)-windows.exe
MACOS_BINARY := $(BINARY_NAME)-macos

.PHONY: all clean setup build-all build-linux build-windows build-macos

# Default target
all: setup build-all

# Create necessary directories
setup:
	@command -v cross >/dev/null 2>&1 || { echo "Installing cross..."; cargo install cross; }
	@mkdir -p $(BUILD_DIR)
	@mkdir -p $(RELEASE_DIR)

# Build for all platforms
build-all: build-linux build-windows build-macos

# Build for Linux
build-linux:
	@echo "Building for Linux..."
	@cross build --release --target $(LINUX_TARGET)
	@cp target/$(LINUX_TARGET)/release/$(BINARY_NAME) $(RELEASE_DIR)/$(LINUX_BINARY)
	@echo "Linux build complete: $(RELEASE_DIR)/$(LINUX_BINARY)"

# Build for Windows
build-windows:
	@echo "Building for Windows..."
	@cross build --release --target $(WINDOWS_TARGET)
	@cp target/$(WINDOWS_TARGET)/release/$(BINARY_NAME).exe $(RELEASE_DIR)/$(WINDOWS_BINARY)
	@echo "Windows build complete: $(RELEASE_DIR)/$(WINDOWS_BINARY)"

# Build for macOS
build-macos:
	@echo "Building for macOS..."
	@cross build --release --target $(MACOS_TARGET)
	@cp target/$(MACOS_TARGET)/release/$(BINARY_NAME) $(RELEASE_DIR)/$(MACOS_BINARY)
	@echo "macOS build complete: $(RELEASE_DIR)/$(MACOS_BINARY)"

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	@rm -rf target/
	@rm -rf $(BUILD_DIR)
	@rm -rf $(RELEASE_DIR)
	@echo "Clean complete"

# Build for specific platform
linux: setup build-linux
windows: setup build-windows
macos: setup build-macos