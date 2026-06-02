# Birdodoro

A lightweight Pomodoro timer with a bird flying animation, built with [Tauri 2.x](https://tauri.app/) (Rust + Web).

When the timer ends, a little bird flies across your screen to remind you to take a break.

## Features

- Pomodoro timer with start/pause/resume/stop
- Customizable work/break durations and cycle length
- Minimizes to system tray, stays out of your way
- Bird animation flies across the screen when time is up
- Auto-cycling: work → short break → long break loop
- Single instance — won't open duplicates
- Persistent settings across restarts
- ~5MB package size

## Screenshots

_TODO: Add screenshots here_

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) >= 18
- [Rust](https://rustup.rs/) >= 1.77
- Windows: [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) (C++ desktop workload)
- Linux: `libgtk-3-dev libwebkit2gtk-4.1-dev libappindicator3-dev`

### Install

```bash
git clone https://github.com/ILTomatoes/birdodoro.git
cd birdodoro
npm install
```

### Development

```bash
cargo tauri dev
```

### Build

```bash
cargo tauri build
```

The installer will be at `src-tauri/target/release/bundle/`.

## Configuration

| Setting | Default | Description |
|---------|---------|-------------|
| Work duration | 25 min | Focus session length |
| Short break | 5 min | Break between pomodoros |
| Long break | 15 min | Break after a full cycle |
| Pomodoros per cycle | 4 | Pomodoros before a long break |
| Bird animation | On | Toggle the flying bird |

## Tech Stack

- **Backend**: Rust ([Tauri 2.x](https://tauri.app/))
- **Frontend**: HTML, CSS, JavaScript (vanilla)
- **Config**: serde_json (JSON file in app data dir)

## License

[MIT](LICENSE)

---

# Birdodoro

一个轻量级番茄钟，带有小鸟飞过动画，基于 [Tauri 2.x](https://tauri.app/)（Rust + Web）构建。

计时结束时，一只小鸟会从屏幕上飞过，提醒你该休息了。

## 功能

- 番茄钟计时，支持开始/暂停/继续/停止
- 可自定义工作时长、休息时长和每组番茄数
- 最小化到系统托盘，不干扰日常工作
- 计时结束时小鸟飞过屏幕动画
- 自动循环：工作 → 短休息 → 长休息
- 单实例运行，不会重复打开
- 设置持久化，重启后保留
- 打包体积仅 ~5MB

## 快速开始

### 环境要求

- [Node.js](https://nodejs.org/) >= 18
- [Rust](https://rustup.rs/) >= 1.77
- Windows: [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)（C++ 桌面开发工作负载）
- Linux: `libgtk-3-dev libwebkit2gtk-4.1-dev libappindicator3-dev`

### 安装

```bash
git clone https://github.com/ILTomatoes/birdodoro.git
cd birdodoro
npm install
```

### 开发

```bash
cargo tauri dev
```

### 构建

```bash
cargo tauri build
```

安装包在 `src-tauri/target/release/bundle/` 目录下。

## 配置

| 设置项 | 默认值 | 说明 |
|--------|--------|------|
| 工作时长 | 25 分钟 | 专注时间 |
| 短休息 | 5 分钟 | 番茄之间的休息 |
| 长休息 | 15 分钟 | 一组完成后的休息 |
| 每组番茄数 | 4 | 进入长休息前的番茄数 |
| 小鸟动效 | 开启 | 开关飞行动画 |

## 技术栈

- **后端**: Rust ([Tauri 2.x](https://tauri.app/))
- **前端**: HTML、CSS、JavaScript（原生）
- **配置**: serde_json（JSON 文件存储在应用数据目录）

## 许可证

[MIT](LICENSE)
