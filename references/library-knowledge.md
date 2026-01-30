# Advanced Tool Configuration

This document explains the underlying libraries used by `image-compress-tool` and their advanced capabilities.

## 1. libwebp (WebP Encoder)

**Library**: `libwebp` (via `webp` crate)
**Purpose**: Next-generation lossy/lossless compression.

### Key Concepts
- **Predictive Coding**: WebP uses values from neighboring blocks to predict values in a new block, then only encodes the difference.
- **Variable Quality**: The `-q` (0-100) parameter controls the aggressiveness of this quantization.
- **Alpha Channel**: WebP supports lossy compression even with transparency (unlike JPEG).

### CLI Flags Mapping
- `--quality <N>`: Maps directly to `WebPConfig.quality`.
- **Method (Speed)**: Hardcoded to `4` (Default) in the Rust tool. Higher values (up to 6) squeeze out ~1% more size but take 2x longer.

## 2. imagequant (libimagequant)

**Library**: `libimagequant` (via `imagequant` crate)
**Purpose**: High-quality palette generation (Color Quantization).

### How it works
1.  **Histogram Analysis**: Analyzes all colors in the image.
2.  **Palette Generation**: Selects the best 256 (or fewer) colors to represent the image using the NeuQuant algorithm.
3.  **Dithering**: Applies Floyd-Steinberg dithering to simulate missing colors, preventing banding in gradients.

### Best Practices
- **Never on Photos**: Applying 256-color limit to a photo looks "retro" or "gif-like". Only use for graphics/UI.
- **Pre-processing**: It effectively converts 32-bit RGBA (4 bytes/pixel) to 8-bit Indexed (1 byte/pixel), instantly reducing raw size by 75% before PNG compression even starts.

## 3. oxipng (PNG Optimizer)

**Library**: `oxipng` (Rust rewrite of OptiPNG)
**Purpose**: Lossless structural optimization of PNG files.

### Optimizations Performed
- **Delta Filters**: Tries different filter types (None, Sub, Up, Average, Paeth) for each scanline to find which one compresses best with DEFLATE.
- **Zopfli Compression**: Uses a stronger DEFLATE algorithm (Zopfli) to reduce the stream size.
- **Strip Metadata**: Removes chunks like `pHYs`, `iCCP`, `tEXt` that aren't needed for display.

### Performance vs Size
- **Level 2 (Default)**: Tries standard filters. Fast.
- **Level 4**: Tries more filter combinations. Slower, but saves extra 5-10%.
- **Level 6**: Brute force. Not recommended for CLI interactive use.
