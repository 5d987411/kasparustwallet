# VSCode Extensions Recommended

For optimal development experience with this Kaspa Rust wallet project, install the following VSCode extensions:

## Required Extensions

1. **rust-analyzer** by rust-lang
   - Provides IDE-like features for Rust
   - Code completion, go to definition, type checking
   - Install from VSCode marketplace

2. **CodeLLDB** by vadimcn  
   - Debugger for C/C++/Rust
   - Required for debugging functionality
   - Install from VSCode marketplace

## Optional Extensions

3. **Better TOML** by bungcip
   - Enhanced syntax highlighting for TOML files
   - Useful for editing Cargo.toml

4. **GitLens** by GitKraken
   - Enhanced git capabilities in VSCode
   - Better blame, history, and repository view

5. **Error Lens** by Alexander
   - Inline display of diagnostics
   - Shows compiler errors directly in code

## Setup Instructions

1. Open VSCode
2. Go to Extensions (Ctrl+Shift+X)
3. Search and install each extension above
4. Reload VSCode after installation
5. Open the kasparustwallet folder

## Project Tasks

Once extensions are installed, you can use these keyboard shortcuts:

- **Ctrl+Shift+P** → "Tasks: Run Task" → Select from:
  - `cargo build` - Build the project
  - `cargo build release` - Build optimized release
  - `cargo run` - Run the wallet CLI
  - `cargo test` - Run tests
  - `cargo check` - Quick syntax/type check
  - `cargo clippy` - Run linter with warnings as errors

- **F5** - Start debugging (requires CodeLLDB)

## Quick Start

1. Open folder in VSCode: `/home/cliff/kasparustwallet`
2. Install recommended extensions
3. Press `Ctrl+Shift+P` and run "Tasks: Run Task"
4. Select "cargo build" to compile the project
5. Use the integrated terminal to run wallet commands

## Troubleshooting

If rust-analyzer shows errors:
- Run `cargo build` in terminal first
- Check that Rust toolchain is installed
- Reload VSCode windows (`Ctrl+Shift+P` → "Developer: Reload Window")

If debugging doesn't work:
- Ensure CodeLLDB is installed
- Check that `.vscode/launch.json` exists
- Try building first with "cargo build"