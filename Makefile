.PHONY: install build serve clean help check-rust

# Load tool versions from .tool-versions file
include .tool-versions
export

help:
	@echo "Available targets:"
	@echo "  check-rust - Verify Rust toolchain version"
	@echo "  install    - Install mdbook and mdbook-metadata"
	@echo "  build      - Build the book"
	@echo "  serve      - Build and serve the book locally"
	@echo "  clean      - Clean build artifacts"
	@echo "  all        - Check Rust, install dependencies and build"

check-rust:
	@echo "Checking Rust version..."
	@rustc --version | grep -q "$(RUST_VERSION)" || \
		(echo "Warning: Expected Rust $(RUST_VERSION), found $$(rustc --version)" && \
		 echo "Run 'rustup install $(RUST_VERSION)' to install the correct version" && \
		 echo "The rust-toolchain.toml file will automatically use the correct version")

install:
	@echo "Installing mdbook $(MDBOOK_VERSION)..."
	@cargo install mdbook --version $(MDBOOK_VERSION)
	@echo "Installing mdbook-metadata $(MDBOOK_METADATA_VERSION)..."
	@cargo install mdbook-metadata --version $(MDBOOK_METADATA_VERSION)
	@echo "Installation complete!"

build:
	@echo "Building the book..."
	@mdbook build

serve:
	@echo "Building and serving the book..."
	@mdbook serve

clean:
	@echo "Cleaning build artifacts..."
	@mdbook clean

all: check-rust install build
