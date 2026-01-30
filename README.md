# ImageCompressSkill ⚡️

[English](README.md) | [简体中文](docs/zh-CN/README.md)

> **The intelligent image optimization skill for AI Agents.**
> 
> A production-grade, Rust-powered engine designed for AI Agents (Gemini, Claude, GPTs) to handle images like pros. Features smart PNG/WebP routing, remote URL processing, and resizing—delivering up to **90% size reduction** with zero configuration.

![AI Skill](https://img.shields.io/badge/AI--Skill-Ready-8A2BE2?style=for-the-badge&logo=openai)
![Rust](https://img.shields.io/badge/Rust-Powered-dea584?style=for-the-badge&logo=rust)
![Docker](https://img.shields.io/badge/Docker-Ready-blue?style=for-the-badge&logo=docker)

## 🤖 Why for AI?

This project is specifically designed to be an **executable skill** for AI Agents (Gemini, Claude, GPTs). It solves the "black box" problem of image compression for LLMs by providing:

1.  **Deterministic Logic**: Hard-coded "Smart Strategy" to decide between PNG/WebP based on content (Photos vs Graphics).
2.  **Safe Execution**: Rust-based binary ensures memory safety and speed.
3.  **Universal Input**: Accepts Local Paths AND **Remote URLs** directly (Agent doesn't need to `curl` first).
4.  **Structured Output**: Returns standardized formats ready for web deployment.

## ✨ Features

- **Dual-Engine Core**:
  - **Photos**: Uses `libwebp` with variable quality control (Default Q=80).
  - **Graphics**: Uses `imagequant` (Index Color) + `oxipng` (Lossless optimization) for crystal clear, tiny PNGs.
- **Smart Resize**: High-quality Lanczos3 resampling with aspect ratio preservation.
- **URL-to-Optimized**: Direct downloading and processing pipeline.
- **Zero-Config**: Sane defaults pre-tuned for 95% of web use cases.
- **Dockerized**: Run anywhere without Rust toolchain.

## 🚀 Usage

### For AI Agents (The Skill)
Inject the [SKILL.md](SKILL.md) context into your agent.

### For Humans (CLI)

**Installation**:
```bash
cargo install --path image-compress-tool
```

**Commands**:
```bash
# Auto-detect best format (usually PNG -> Optimized PNG)
image-compress-tool -i input.png -o output.png

# Force WebP (Best for photos)
image-compress-tool -i photo.jpg -o photo.webp --quality 85

# Resize & Compress from URL
image-compress-tool -i "https://site.com/img.jpg" -o thumb.webp --width 200
```

### via Docker
No Rust installed? No problem.

```bash
docker build -t compress-pro -f docker/Dockerfile .
docker run -v $(pwd):/data compress-pro -i /data/input.png -o /data/output.webp
```

## 🧠 Smart Strategy (How it works)

| Input Type | Strategy | Toolchain | Result |
| :--- | :--- | :--- | :--- |
| **Photographs** | Lossy WebP | `image` -> `libwebp` | ~90% smaller |
| **UI / Logos** | Optimized PNG | `imagequant` -> `oxipng` | ~70% smaller |
| **Simple Shapes** | Quantized PNG | `imagequant` (256 colors) | Tiny file, crisp edges |

## 🛠 Project Structure

- `image-compress-tool/`: Rust source code.
- `SKILL.md`: **AI Skill Definition** (Prompts & Rules).
- `docker/`: Deployment configuration.
- `web_test/`: Visual regression & compression benchmark gallery.

## 📄 License

MIT
