# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-01-17

### Added
- Initial standalone release of evm-cli
- Interactive fuzzy filter UI for method selection
- Context menu for contract management
- Session persistence with automatic state saving
- Built-in Solidity compiler integration
- Support for all Solidity parameter types
- Transaction management (view, send, payable methods)
- .env file support for secure credential management
- Comprehensive PRD documentation

### Changed
- Extracted from cargo-pvm-contract workspace into standalone repository
- Version reset to 0.1.0 for independent versioning
- Cargo.toml edition corrected to 2021 (from invalid 2024)

## History

This tool was originally developed as part of the [cargo-pvm-contract](https://github.com/paritytech/cargo-pvm-contract) workspace with the following commits:

- `b0dd17f` - Overhaul evm-cli filter UI with bottom-anchored menu and method logging
- `7b514f9` - Improve evm-cli UX: graceful cancellation, header fix, and add PRD
- `c7cd70c` - Add evm-cli: interactive CLI for Solidity contract deployment
