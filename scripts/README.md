# EVM-CLI Scripts Directory

This directory contains utility scripts for evm-cli development and QA.

## Available Scripts

### record-spec-tape.sh

**Purpose:** Record VHS tapes for evm-cli specs and generate ASCII review files

**Type:** QA / Documentation

**Usage:**
```bash
./record-spec-tape.sh <spec-name> <feature-name>
```

**Examples:**
```bash
./record-spec-tape.sh main-interface layout-and-zones
./record-spec-tape.sh contracts-menu load-contract
./record-spec-tape.sh ctrl-p-menu palette-basics
./record-spec-tape.sh tx-and-call-popup parameter-input
./record-spec-tape.sh output-panel card-navigation
./record-spec-tape.sh general-settings keyboard-shortcuts
```

**What it does:**
1. ✅ Validates prerequisites (VHS, solc, blockchain)
2. ✅ Builds the project in release mode
3. ✅ Executes VHS recording
4. ✅ Generates ASCII file (`.ascii`) for terminal review
5. ✅ Generates MP4 video (`.mp4`) for playback
6. ✅ Creates review guide (`.REVIEW.md`)
7. ✅ Backs up and restores user config

**Outputs:**
```
spec/<spec-name>/recordings/
├── <feature>.tape       # VHS script
├── <feature>.ascii      # ASCII recording (text-based)
├── <feature>.mp4        # MP4 video
└── <feature>.REVIEW.md  # Review guide
```

**Prerequisites:**
- VHS (Terminal recorder): `brew install charmbracelet/tap/vhs`
- Solc (Compiler): `brew install solidity`
- Rust & Cargo: For building the project
- Anvil (optional): `anvil` for blockchain simulation

**For Complete Documentation:**

See [spec/QA_EXPERT_GUIDE.md](../spec/QA_EXPERT_GUIDE.md) for:
- Full prerequisite setup instructions
- Step-by-step recording workflow
- How to review ASCII files
- Comprehensive review checklist
- Troubleshooting guide
- Best practices for QA

## Script Organization

```
scripts/
├── README.md                  # This file
├── record-spec-tape.sh       # Record VHS tapes for QA
└── (future scripts here)
```

## Contributing

When adding new scripts:

1. Make scripts executable: `chmod +x script.sh`
2. Add comprehensive help text: `--help` flag
3. Use consistent color output (RED, GREEN, YELLOW, BLUE)
4. Include error handling and validation
5. Document prerequisites
6. Add usage examples
7. Update this README

## Quick Links

- **QA Recording Guide:** [spec/QA_EXPERT_GUIDE.md](../spec/QA_EXPERT_GUIDE.md)
- **Recording Manifest:** [spec/RECORDINGS_MANIFEST.md](../spec/RECORDINGS_MANIFEST.md)
- **Specs Directory:** [spec/](../spec/)
- **VHS Documentation:** https://github.com/charmbracelet/vhs
