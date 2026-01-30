# ImageCompressSkill ⚡️

> **面向 AI Agent 的智能图像优化技能 (AI Skill)。**
> 
> 为 AI Agent (Gemini, Claude, GPTs) 量身定制的生产级 Rust 引擎。支持智能 PNG/WebP 路由、远程 URL 处理及缩放——在零配置的情况下实现最高 **90% 的体积缩减**。

## 🤖 为什么选择此 AI Skill？

本项目专门为 AI Agent（Gemini, Claude, GPTs）设计，旨在成为一个**可执行技能 (Executable Skill)**。它通过以下方式解决了大模型处理图像压缩时的“黑盒”问题：

1.  **确定性逻辑**: 内置“智能策略”，自动根据内容（照片 vs 图形）决定使用 PNG 还是 WebP。
2.  **安全执行**: 基于 Rust 的二进制文件，确保内存安全与极致性能。
3.  **通用输入**: 直接支持本地路径和 **远程 URL**（Agent 无需先运行 `curl`）。
4.  **结构化输出**: 返回符合生产标准的格式，可直接用于 Web 部署。

## ✨ 核心特性

- **双引擎核心**:
  - **照片类**: 使用 `libwebp` 进行有损压缩，质量可控（默认 Q=80）。
  - **图形类**: 使用 `imagequant`（索引色）+ `oxipng`（无损优化），生成极小且清晰的 PNG。
- **智能缩放**: 采用高质量的 Lanczos3 重采样算法，自动保持纵横比。
- **URL 处理流水线**: 直接下载并一键优化远程图片。
- **零配置**: 预设参数已针对 95% 的 Web 场景进行深度调优。
- **Docker 化**: 无需 Rust 环境即可在任何地方运行。

## 🚀 使用指南

### 针对 AI Agent (作为 Skill)
将 `SKILL.md` 的上下文注入到您的 Agent 中即可。

### 针对人类 (CLI)

**安装**:
```bash
cargo install --path image-compress-tool
```

**命令示例**:
```bash
# 自动探测最佳格式 (通常 PNG -> 优化后的 PNG)
image-compress-tool -i input.png -o output.png

# 强制 WebP (适合照片)
image-compress-tool -i photo.jpg -o photo.webp --quality 85

# 从 URL 缩放并压缩
image-compress-tool -i "https://site.com/img.jpg" -o thumb.webp --width 200
```

### 通过 Docker
```bash
docker build -t compress-pro -f docker/Dockerfile .
docker run -v $(pwd):/data compress-pro -i /data/input.png -o /data/output.webp
```

## 🧠 智能压缩策略 (工作原理)

| 输入类型 | 策略 | 工具链 | 预期效果 |
| :--- | :--- | :--- | :--- |
| **真实照片** | 有损 WebP | `image` -> `libwebp` | 缩减 ~90% |
| **UI / Logo** | 优化版 PNG | `imagequant` -> `oxipng` | 缩减 ~70% |
| **简单图形** | 量化版 PNG | `imagequant` (256色) | 极小体积，边缘锐利 |

## 🛠 项目结构

- `image-compress-tool/`: Rust 源代码。
- `SKILL.md`: **AI Skill 定义** (包含 Prompts 与 规则)。
- `docker/`: 部署配置。
- `web_test/`: 视觉回归与压缩基准测试展示。

## 📄 开源协议

MIT
