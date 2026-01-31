# Application Icons

This directory contains the application icons for different platforms.

## Required Icons

For Tauri applications, you need the following icon files:

- `icon.png` - Main icon (512x512 recommended)
- `icon.ico` - Windows icon
- `icon.icns` - macOS icon
- `32x32.png` - Small icon
- `128x128.png` - Medium icon
- `128x128@2x.png` - Retina display icon (256x256)

## Generating Icons

You can use the Tauri CLI to generate icons from a single source:

```bash
# Install tauri-cli if not already installed
cargo install tauri-cli

# Generate icons from a 1024x1024 PNG source
cargo tauri icon path/to/source-icon.png
```

## Current Status

The icons in this directory are placeholders. Replace them with your actual branding icons before building for production.
