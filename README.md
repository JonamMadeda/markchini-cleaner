# MarkChini

**A local-first Markdown-to-PDF desktop app** — zero external API calls, instant live preview, native PDF export via headless Chromium.

![License](https://img.shields.io/badge/license-MIT-blue)
[![Download Latest](https://img.shields.io/github/v/release/JonamMadeda/markchini-cleaner?label=Download&color=blue)](https://github.com/JonamMadeda/markchini-cleaner/releases/latest)

## Features

- **Split-screen editor** with live preview on every keystroke
- **Export to PDF** via headless Chromium (Google Chrome or Microsoft Edge)
- **Professional typography** — Georgia/Times New Roman serif, A4 layout, 2.5cm margins
- **Multi-page tables** with repeating headers and page-break avoidance
- **Blockquotes, lists, code blocks** styled for print with break prevention
- **Dark/light mode** toggle persisted in localStorage
- **Image insertion** via native file picker (Ctrl+Shift+I)
- **Keyboard shortcuts**: Ctrl+O (open), Ctrl+S (save), Ctrl+P (export PDF)
- **Resizable panes** via drag divider
- **Font family/size/margin** dropdowns
- **100% local** — no network requests, no data leaves your machine
- **Automatic updates** via GitHub Releases

## Installation

### Windows

1. Go to the [latest release](https://github.com/JonamMadeda/markchini-cleaner/releases/latest)
2. Download the `.msi` installer
3. Run the installer — it will register MarkChini in "Add or Remove Programs"
4. Launch from Start Menu or Desktop shortcut

**Prerequisites:**
- Windows 10 or 11 (WebView2 Runtime pre-installed)
- Google Chrome or Microsoft Edge (for PDF export; Edge is pre-installed on Win10/11)

### From source

```powershell
# Install Rust (https://rustup.rs)
# Install WiX Toolset (v3 via Chocolatey)
choco install wixtoolset --version 3.14.1 -y

# Clone and build
git clone https://github.com/JonamMadeda/markchini-cleaner.git
cd markchini-cleaner/src-tauri
cargo tauri build --bundles msi
```

## Usage

1. **Write Markdown** in the left editor pane
2. **Preview updates** live in the right pane
3. **Export PDF** — click the export button (or Ctrl+P), choose a save location
4. **Customize** font family, size, and margins via the toolbar dropdowns

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| Ctrl+O | Open Markdown file |
| Ctrl+S | Save Markdown file |
| Ctrl+P | Export to PDF |
| Ctrl+Shift+I | Insert image |

## Updating

MarkChini checks for updates automatically on startup. When a new version is available:

1. A native dialog prompts you to download and install
2. The update downloads silently in the background
3. The app restarts with the new version installed

You can also download the latest `.msi` from the [releases page](https://github.com/JonamMadeda/markchini-cleaner/releases) and install manually — the installer replaces the previous version.

## Architecture

```
Markdown ──► pulldown-cmark ──► HTML + CSS ──► headless_chrome ──► PDF bytes ──► file write
                  │
                  ▼
           Live preview (marked.js)
```

- **Rust backend**: `tauri` v2, `pulldown-cmark` 0.13, `headless_chrome` 1.0
- **Frontend**: Vanilla HTML/JS, Tailwind CSS (CDN), `marked.js` for preview rendering
- **PDF engine**: Headless Chromium via Chrome DevTools Protocol
- **Bundling**: WiX Toolset (.msi installer)
- **Updates**: `tauri-plugin-updater` via GitHub Releases

## Build

```powershell
cd src-tauri
cargo tauri build --bundles msi
```

Output: `src-tauri/target/release/bundle/msi/MarkChini_1.0.0_x64_en-US.msi`

## License

MIT
