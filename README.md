# SSM Splitscreen Minecraft for Steamdeck!

> ⚠️ **Pre-Alpha / Under Development**  
> This project is in early development and undergoes frequent changes. APIs, features, and behavior may change without notice. Functionality is not done, so this tool won't work yet!

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](#)
[![License](https://img.shields.io/badge/license-CC%20BY--SA-blue)](#license)
[![Rust](https://img.shields.io/badge/rust-2024_edition-orange)](#)

A tool to automatically set up local splitscreen multiplayer sessions for Minecraft on the Steam Deck. Detects connected controllers, splits the screen accordingly, and launches multiple Minecraft instances paired with individual controllers—all through a single command.

> **Note**: This tool is designed exclusively for the Steam Deck. It requires Steam Deck-specific features and will not work on other platforms.

## 🎮 Features

- **Automatic Controller Detection** - Detects 1-4 connected controllers (including built-in Steam Deck controller when no external controllers detected)
- **Smart Screen Splitting** - Automatically calculates optimal window layouts for 1, 2, 3, or 4 players
- **PolyMC Integration** - Works with PolyMC for Minecraft instance management
- **Desktop Integration** - Creates Steam desktop entry for easy launching
- **Configuration Management** - Persistent configuration with JSON storage
- **Steam Deck Optimized** - Built specifically for Steam Deck UI and hardware

## 📋 Prerequisites

**Steam Deck (Required)**
- **OS**: SteamOS (Steam Deck OS)
- **Rust**: 1.70+ (for building from source)

**Software Dependencies**:
- PolyMC (flatpak installation)


## 🚀 Quick Start

### Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/ssm-splitscreen-minecraft.git
cd ssm-splitscreen-minecraft

# Build the project
cargo build --release

# Run
./target/release/ssm_splitscreen
```

### Installation on Steam Deck

1. Build the project or download a pre-compiled binary from the releases on GitHub
2. Copy to any location you want, no `root` or `sudo` needed
3. Launch on first run to complete initial setup
4. The application will guide you to create a Steam library entry for easy access

## 📖 Usage

### First Run
```bash
ssm_splitscreen
```
During first run:
1. Enter your desired Minecraft version (e.g., 1.21.11)
2. A desktop entry will be created
3. Add as a non-Steam game to Steam if desired


### Subsequent Runs
```bash
ssm_splitscreen
```
The application will:
1. Detect connected controllers
2. Calculate optimal screen layout
3. Launch configured Minecraft instances
4. Assign each instance to a controller

### Configuration
Configuration is stored at `~/.ssm/config.json`:
```json
{
  "minecraft_version": "1.21.11",
  "initial_setup": true
}
```

## 🏗️ Project Structure

```
src/
├── main.rs              # Application entry point and orchestration
├── config.rs            # Configuration management
├── logger.rs            # Logging utilities
├── minecraft_version.rs # Version validation and parsing
├── polymc_helper.rs     # PolyMC integration
├── system.rs            # Hardware and system detection
├── setup.rs             # Initial setup and configuration prompts
└── window_layout.rs     # Window positioning calculations
```

## 🧪 Testing

This project has comprehensive test coverage using `rstest`:

```bash
# Run all tests
cargo test

# Run specific test module
cargo test minecraft_version
cargo test config
cargo test logger
cargo test window_layout
```

## 🔍 Code Quality

```bash
# Run clippy linter
cargo clippy -- -D warnings

# Format code
cargo fmt --all

# Generate documentation
cargo doc --open
```

## 🚧 Roadmap

- [ ] PolyMC instance creation and configuration
- [ ] Minecraft instance launching with window arguments
- [ ] Controller pairing verification
- [ ] Save/load game world sync across instances
- [ ] Pre-built binaries for Steam Deck
- [ ] Steam Deck-specific UI enhancements

## 🤝 Contributing

Contributions are welcome! Please ensure:

1. **Tests** - All new code includes unit tests using `rstest`
2. **Quality** - Code passes clippy with `-D warnings`
3. **Style** - Follows Rust 2024 edition conventions
5. **Verification** - Run the full check suite before submitting:
   ```bash
   cargo build --quiet
   cargo test --quiet
   cargo clippy --quiet -- -D warnings
   cargo doc --quiet
   cargo fmt --all --quiet
   ```

## 📝 License

This project is licensed under the Creative Commons Attribution-ShareAlike 4.0 International License (CC BY-SA) - see the [LICENSE](LICENSE) file for details.

## 🙋 Support

For issues, questions, or suggestions:
- Open an [Issue](https://github.com/scspijker/ssm-splitscreen-minecraft/issues)
- Start a [Discussion](https://github.com/scsp[ijker/ssm-splitscreen-minecraft/discussions)

## 📖 Resources

- [Steam Deck Documentation](https://github.com/ValveSoftware/SteamDeck-ProjectFiles)
- [PolyMC Project](https://polymc.org/)
- [Rust Book](https://doc.rust-lang.org/book/)

---

**Made with ❤️ for Steam Deck gamers that enjoy gaming together**
