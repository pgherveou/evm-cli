# Contract Compilation

This document describes how Solidity contracts are compiled in evm-cli.

## Overview

evm-cli uses the `solc` (Solidity compiler) command-line tool to compile `.sol` files. The compilation process extracts the ABI and bytecode needed for deployment and interaction.

## Prerequisites

- **solc** must be installed and available in your PATH
- Install via: `npm install -g solc` or from [solidity releases](https://github.com/ethereum/solidity/releases)

## Compilation Process

### 1. solc Invocation

The compiler is invoked with:

```bash
solc --combined-json abi,bin <path-to-sol-file>
```

This produces a JSON output containing:
- `contracts`: Map of `"filename:ContractName"` → `{ abi, bin }`
- `errors`: Array of compilation errors/warnings

### 2. Output Parsing

The JSON output is parsed to extract:

| Field | Description |
|-------|-------------|
| `abi` | Contract ABI as JSON, parsed into `JsonAbi` |
| `bin` | Hex-encoded bytecode, decoded to `Vec<u8>` |

### 3. Result

Each contract in the file produces a `CompiledContract`:

```rust
pub struct CompiledContract {
    pub name: String,      // Contract name (e.g., "Counter")
    pub abi: JsonAbi,      // Parsed ABI for encoding/decoding
    pub bytecode: Vec<u8>, // Deployment bytecode
}
```

## Error Handling

| Error | Cause |
|-------|-------|
| `Failed to execute solc` | solc not installed or not in PATH |
| `solc compilation failed` | Syntax errors in Solidity code |
| `Failed to parse solc output` | Unexpected solc output format |
| `No contracts found` | File contains no contract definitions |

## Multiple Contracts

If a Solidity file contains multiple contracts:
1. All contracts are compiled
2. User is prompted to select which contract to use
3. Selection is made via the contract selector popup
