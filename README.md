# Wolai 日历图标下载器

批量下载 Wolai 日历图标的桌面工具，支持 Windows 和 macOS。

## 功能

- 选择日期区间（起始日期 ~ 结束日期）
- 选择图标颜色（红/蓝/绿/黄/粉）
- 批量下载 SVG 格式日历图标
- 按年份自动分目录保存
- 实时显示下载进度

## 下载

前往 [Releases](../../releases) 页面下载对应平台的安装包：

- **Windows**: `.msi` 或 `.exe` 安装包
- **macOS**: `.dmg` 安装包
- **Linux**: `.deb` 或 `.AppImage`

## 图标链接格式

下载后的图标按以下结构保存：

```
保存目录/
├── 2024/
│   ├── 01-01.svg
│   ├── 01-02.svg
│   └── ...
├── 2025/
│   └── ...
```

## 开发

需要 Rust 和 Bun 环境。

```bash
bun install
bun run tauri dev
```

## 构建

```bash
bun run build
```

产物在 `src-tauri/target/release/bundle/` 目录下。
