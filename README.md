# ImageCompressPro ⚡️

[English](#english) | [简体中文](#chinese)

<a name="english"></a>

> **A production-grade, Rust-powered image optimization engine.**
> 
> Intelligently routes images between aggressive PNG quantization (ImageQuant + Oxipng) and next-gen WebP compression based on content type, delivering up to **90% size reduction** without visual compromise.

![Rust](https://img.shields.io/badge/built_with-Rust-dca282.svg)
![Docker](https://img.shields.io/badge/docker-ready-blue.svg)

## ✨ Features

- **Dual-Engine Core**:
  - **Photos**: Uses `libwebp` with variable quality control (Default Q=80).
  - **Graphics**: Uses `imagequant` (Index Color) + `oxipng` (Lossless optimization) for crystal clear, tiny PNGs.
- **Smart Resize**: High-quality Lanczos3 resampling with aspect ratio preservation.
- **URL-to-Optimized**: Direct downloading and processing pipeline.
- **Zero-Config**: Sane defaults pre-tuned for 95% of web use cases.
- **Dockerized**: Run anywhere without Rust toolchain.

## 🚀 Usage

### Installation

```bash
cargo install image-compress-pro
```

### Commands

```bash
# Auto-detect best format (usually PNG -> Optimized PNG)
image-compress-pro -i input.png -o output.png

# Force WebP (Best for photos)
image-compress-pro -i photo.jpg -o photo.webp --quality 85

# Resize & Compress from URL
image-compress-pro -i "https://site.com/img.jpg" -o thumb.webp --width 200
```

### via Docker

```bash
docker build -t compress-pro .
docker run -v $(pwd):/data compress-pro -i /data/input.png -o /data/output.webp
```

---

<a name="chinese"></a>

# 图像压缩专业版 (ImageCompressPro)

> **生产级 Rust 图像优化引擎。**
> 
> 根据内容类型（照片或图形）智能路由：采用激进的 PNG 量化（ImageQuant + Oxipng）或新一代 WebP 压缩。在不牺牲视觉质量的前提下，实现最高 **90% 的体积缩减**。

## ✨ 核心特性

- **双引擎核心**:
  - **照片类**: 使用 `libwebp` 进行有损压缩，质量可控（默认 Q=80）。
  - **图形类**: 使用 `imagequant`（索引色）+ `oxipng`（无损优化），生成极小且清晰的 PNG。
- **智能缩放**: 采用高质量的 Lanczos3 重采样算法，自动保持纵横比。
- **URL 处理流水线**: 直接下载并一键优化远程图片。
- **零配置**: 预设参数已针对 95% 的 Web 场景进行深度调优。
- **Docker 化**: 无需 Rust 环境即可在任何地方运行。

## 🚀 使用指南

### 安装

```bash
cargo install image-compress-pro
```

### 命令示例

```bash
# 自动探测最佳格式 (通常 PNG -> 优化后的 PNG)
image-compress-pro -i input.png -o output.png

# 强制 WebP (适合照片)
image-compress-pro -i photo.jpg -o photo.webp --quality 85

# 从 URL 缩放并压缩
image-compress-pro -i "https://site.com/img.jpg" -o thumb.webp --width 200
```

### 通过 Docker

```bash
docker build -t compress-pro .
docker run -v $(pwd):/data compress-pro -i /data/input.png -o /data/output.webp
```

## 📄 License

MIT
