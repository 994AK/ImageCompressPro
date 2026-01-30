# Plan: Image Compress Tool Setup

**Generated**: 2026-01-30
**Estimated Complexity**: Medium

## Overview
Create a Rust CLI tool that compresses images using `imagequant` (for lossy quantization) and `oxipng` (for lossless PNG optimization). This tool will allow users to compress images via command line.

## Prerequisites
- Rust toolchain (cargo, rustc)
- `imagequant` crate dependencies (may require C library or just crate)
- `oxipng` crate

## Sprint 1: Project Initialization & Basic CLI
**Goal**: Set up the project structure and handle command-line arguments.
**Status**: Completed
**Demo/Validation**:
- Run `cargo run -- --help` and see the help message.

### Task 1.1: Initialize Rust Project
- **Status**: Completed

### Task 1.2: Add Dependencies
- **Status**: Completed

### Task 1.3: Implement CLI Arguments
- **Status**: Completed

## Sprint 2: Image Compression Logic
**Goal**: Implement the core compression pipeline: Load -> Quantize -> Optimize -> Save.
**Status**: Completed
**Demo/Validation**:
- Compress a sample PNG image and verify size reduction.

### Task 2.1: Image Loading and Quantization
- **Status**: Completed

### Task 2.2: PNG Encoding and Optimization
- **Status**: Completed

## Sprint 3: Resize and WebP Support
**Goal**: Add resizing capabilities and WebP output format support.
**Demo/Validation**:
- Resize a large image to 200x200 and save as WebP.

### Task 3.1: Add Resize Arguments
- **Location**: `src/main.rs`
- **Description**: Add `--max-width` and `--max-height` (or `--size`) arguments.
- **Complexity**: 2
- **Dependencies**: Task 1.3
- **Acceptance Criteria**:
  - CLI accepts resize dimensions.
  - Image is resized using `image::imageops::resize` (FilterType::Lanczos3) before processing.
  - Maintains aspect ratio (resize to fit).
- **Validation**: Compress and resize an image, check dimensions.

### Task 3.2: Support WebP Output
- **Location**: `src/main.rs`
- **Description**: Detect output format from extension. If `.webp`, use `image` crate's WebP encoder instead of `imagequant`/`oxipng` pipeline.
- **Complexity**: 3
- **Dependencies**: Task 2.1
- **Acceptance Criteria**:
  - If output file ends in `.webp`, save as WebP.
  - Skips `imagequant`/`oxipng` for WebP (unless we want to quantize before WebP, but standard WebP usually handles it).
- **Validation**: Run with output filename ending in `.webp`, check file format.

## Testing Strategy
- Unit tests for helper functions.
- Integration test: Compress a known sample image, check if output exists and is valid PNG.

## Potential Risks & Gotchas
- `imagequant` might require system libraries (libimagequant-dev) depending on the crate version/features. I will try to use the pure Rust or bundled version if available, or just the standard crate which usually handles bindings.
- `image` crate compatibility with `imagequant` input types (need raw pointers or slice conversion).

## Rollback Plan
- `rm -rf image-compress-tool` to restart.
