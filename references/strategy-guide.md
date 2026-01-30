# Usage Scenarios & Strategy Guide

This guide helps the agent decide which compression strategy to use based on the input image content and user intent.

## 1. Strategy Matrix

| Content Type | Characteristics | Recommended Strategy | Why? |
| :--- | :--- | :--- | :--- |
| **Photographs** | Continuous tones, complex gradients, no sharp edges. (e.g., Camera photos, realistic renders) | **WebP (Lossy)** | WebP's predictive coding excels at photos, reducing size by 80-90% vs JPEG/PNG. |
| **Screenshots (UI)** | Mix of text, flat colors, and some gradients. Sharp edges are critical. | **Optimized PNG** | Quantization reduces colors smartly while `oxipng` preserves sharp edges of text/UI elements. |
| **Logos / Icons** | Few colors (< 256), sharp vector-like edges, transparency. | **Quantized PNG** | ImageQuant forces a palette (indexed color), resulting in tiny files with perfect crispness. |
| **Line Art / Drawings** | Solid strokes, limited palette. | **Optimized PNG** | Similar to Logos, indexed color is far superior to lossy compression here. |
| **Universal Web** | General purpose web assets where bandwidth is key. | **WebP (Lossy)** | Best overall compatibility/size ratio for modern web. |

## 2. Parameter Tuning Guide

### WebP Tuning (`--quality`)

- **Q = 80 (Default)**: The "Gold Standard" for web. Indistinguishable from original for 99% of viewers.
- **Q = 75**: Aggressive. Use for thumbnails or mobile-only assets. Minor artifacts may appear in dark areas.
- **Q = 90**: High Fidelity. Use for hero images or photography portfolios.
- **Q = 100**: Near-lossless (but still lossy). Only use if archiving.

### PNG Tuning (Internal Logic)

The tool automatically handles this via `imagequant` + `oxipng`.
- **Quality**: The tool targets min 0 / max 80 quality for quantization.
- **Optimization**: Uses `oxipng` level 2 (balanced) to level 4 (production).

## 3. When to Resize?

- **Thumbnails**: Width `200-400px`.
- **Blog Content**: Width `800-1200px`.
- **Full Screen / Hero**: Width `1920px` (or original).

**Rule of Thumb**: Always ask if the user has a target display size. If unknown, keep original resolution or cap at `1920px` for web safety.
