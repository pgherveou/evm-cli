# Feature: Forge-based Compilation with EVM/PVM Support

## Summary

Rework contract compilation to use `forge build` instead of direct `solc` invocation, enabling support for both EVM and PVM (PolkaVM) bytecode compilation.

## Motivation

- **Forge integration**: Leverage Foundry's build system for better dependency management and project compatibility
- **Dual target support**: Enable deployment to both EVM chains and PVM-based chains (Polkadot/JAM)
- **resolc support**: Use `--resolc-compile` flag to compile Solidity to PVM bytecode via resolc

## Current Behavior

Currently, `evm-cli` directly invokes `solc`:

```bash
solc --combined-json abi,bin <path-to-sol-file>
```

This only produces EVM bytecode and doesn't integrate with Foundry projects.

## Proposed Changes

### 1. Compilation with Forge

Replace direct `solc` calls with `forge build <path/to/contract.sol>`:

**For EVM bytecode:**
```bash
forge build <path/to/contract.sol>
```

**For PVM bytecode:**
```bash
forge build --resolc-compile <path/to/contract.sol>
```

This works for both standalone `.sol` files and Foundry projects.

### 2. Artifact Location

Forge outputs compiled artifacts to the same location for both targets:
- `out/<ContractName>.sol/<ContractName>.json`

**Note:** Resolc compilation overwrites the same `out/` directory as EVM compilation. To support both targets, use the `-o` flag to specify different output directories:
- EVM: `forge build -o out-evm <path>`
- PVM: `forge build --resolc-compile -o out-pvm <path>`

PVM bytecode can be identified by its magic prefix: `0x50564d00` ("PVM\0" in ASCII).

Each artifact JSON contains:
- `abi`: Contract ABI
- `bytecode.object`: Deployment bytecode
- `deployedBytecode.object`: Runtime bytecode

### 3. Deployment Popup Changes

The deployment popup must now **always** be shown when deploying a contract, with:

| Field | Type | Description |
|-------|------|-------------|
| **Bytecode Target** | Selector | `EVM` (default) or `PVM` |
| Constructor Args | Input fields | Based on ABI (optional, as before) |

**UI Flow:**
1. User selects a contract to deploy
2. Deployment popup appears with:
   - Bytecode target selector (EVM/PVM) at the top
   - Constructor arguments below (if any)
3. On confirm, compile with appropriate flags if needed, then deploy

### 4. Compilation Strategy

Compile on demand when user confirms deployment:
- When user selects bytecode target and confirms, compile with appropriate flags
- Use the output panel to show compilation status (message before and after compilation)
- Only compile what's needed for the selected target

### 5. Data Model Changes

```rust
pub enum BytecodeTarget {
    Evm,
    Pvm,
}

pub struct CompiledContract {
    pub name: String,
    pub abi: JsonAbi,
    pub evm_bytecode: Option<Vec<u8>>,  // Compiled with forge build
    pub pvm_bytecode: Option<Vec<u8>>,  // Compiled with forge build --resolc-compile
}
```

Or alternatively, keep bytecode as a single field and track which target it was compiled for.

## Implementation Tasks

- [ ] Replace `solc` invocation with `forge build` subprocess
- [ ] Add `--resolc-compile` support for PVM compilation
- [ ] Parse forge output artifacts (JSON files in `out/` directory)
- [ ] Update `CompiledContract` struct to support dual bytecode targets
- [ ] Create bytecode target selector widget
- [ ] Modify deployment popup to always show with target selector
- [ ] Add compilation caching to avoid redundant builds
- [ ] Update error handling for forge-specific errors
- [ ] Update documentation

## Dependencies

- `forge` (Foundry) must be installed
- `resolc` must be installed for PVM compilation
