# Project Overview

SSM Splitscreen Minecraft is a tool to setup splitscreen sessions for Minecraft on the Steamdeck. It automatically detects the number of connected controllers and starts 1-4 Minecraft's, diving them over the screen and pairing them with a controller. To do this it integrates with PolyMC.

# Style

## General

* You must follow Rust 2024 edition conventions
* You must write unit tests using `rstest` for new code
	* You must cover happy paths and edge cases
* You must use git mv instead of other methods to keep Git history intact!
* Prefer modularity over large files
* Prefer immutability
* Prefer functional over imperative
* Prefer existing dependencies over adding new ones when possible.
* For complex code, always consider using third-party libraries instead of writing new code that has to be maintained.

## Security

* Always write secure code.
* Never hardcode sensitive data.
* Do not log sensitive data.
* All user input must be validated.
* Never roll your own cryptography system.

## Production Ready

* All generated code must be production ready.
* There must be no stubs "for production".
* There must not be any non-production logic branches in the main code package itself.
* Any code or package differences between Development and Production must be avoided unless absolutely necessary.


# Post-Change Verification

Run after code changes:
```bash
cargo build --quiet
cargo test --quiet
cargo clippy --quiet -- -D warnings
cargo doc --quiet
cargo fmt --all --quiet
```

# Additional Context

Skills provide task-specific guidance. Use the `skill` tool to load one when needed - available skills are listed in its description.