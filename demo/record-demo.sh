#!/bin/bash
# Demo script for evm-cli - can test or record
# Run this from within a tmux session

set -e

# Parse arguments
RECORD=false
UPLOAD=false

while [[ $# -gt 0 ]]; do
	case $1 in
	--record)
		RECORD=true
		shift
		;;
	--upload)
		UPLOAD=true
		shift
		;;
	*)
		echo "Unknown option: $1"
		echo "Usage: $0 [--record] [--upload]"
		exit 1
		;;
	esac
done

if $RECORD; then
	echo "🎬 evm-cli Demo Recording"
	echo "========================="
else
	echo "🎬 evm-cli Demo Testing"
	echo "======================="
fi
echo ""

# Check if we're in a tmux session
if [ -z "$TMUX" ]; then
	echo "❌ Not in a tmux session!"
	echo ""
	echo "Start tmux first:"
	echo "   tmux"
	exit 1
fi

# Check prerequisites
if ! command -v solc &>/dev/null; then
	echo "❌ solc not found. Install: sudo pacman -S solidity"
	exit 1
fi

if $RECORD; then
	if ! command -v asciinema &>/dev/null; then
		echo "❌ asciinema not found. Install: sudo pacman -S asciinema"
		exit 1
	fi

	if ! command -v agg &>/dev/null; then
		echo "❌ agg not found (needed for screenshot generation)"
		echo "Install with: cargo install --git https://github.com/asciinema/agg"
		exit 1
	fi
fi

if $UPLOAD && ! command -v asciinema &>/dev/null; then
	echo "❌ asciinema not found (needed for --upload). Install: sudo pacman -S asciinema"
	exit 1
fi

# Check blockchain
if ! curl -s -X POST -H "Content-Type: application/json" \
	--data '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' \
	http://localhost:8545 >/dev/null 2>&1; then
	echo "❌ No blockchain at http://localhost:8545"
	echo "Start anvil in another pane"
	exit 1
fi

# Build
echo "Building..."
cargo build --release
echo "✅ Build complete"
echo ""

# Create output directory if recording
if $RECORD; then
	mkdir -p docs
fi

# Get current pane
CURRENT_PANE=$(tmux display-message -p '#P')

# Create a new pane for the demo
echo "Creating demo pane..."
tmux split-window -h

# Get the demo pane ID
DEMO_PANE=$(tmux display-message -p '#P')

# Focus back on the control pane
tmux select-pane -t "$CURRENT_PANE"

echo ""
if $RECORD; then
	echo "📹 Recording demo in pane $DEMO_PANE"
else
	echo "🎬 Running demo in pane $DEMO_PANE"
fi
echo "   Watch the automation!"
echo ""
sleep 1

# Helper functions
send_keys() {
	local keys="$1"
	local delay="$2"
	tmux send-keys -t "$DEMO_PANE" "$keys"
	sleep "$delay"
}

send_enter() {
	local delay="$1"
	tmux send-keys -t "$DEMO_PANE" Enter
	sleep "$delay"
}

log_step() {
	echo "✓ $1"
}

# Start asciinema if recording
if $RECORD; then
	send_keys "asciinema rec --overwrite --title 'evm-cli: Interactive Solidity Contract Deployment' docs/demo.cast" 0.5
	send_enter 2
fi

# Start the app
send_keys "./target/release/evm-cli" 0.5
send_enter 1

# Execute demo steps
log_step "Loading contract: examples/Demo.sol"
send_enter 1
send_keys "examples/Demo.sol" 0.5
send_enter 1

log_step "Deploying contract with constructor arg: 42"
send_keys "Down" 0.3
send_enter 0.5
send_keys "42" 0.5
send_enter 0.5

log_step "Calling getCount() view function"
send_keys "Down" 0.3
send_enter 1.5

log_step "Calling increment() state-changing function"
send_keys "Down" 0.3
send_enter 1.5

log_step "Calling getCount() again"
send_keys "Up" 0.3
send_enter 1.5

if $RECORD; then
	# Final pause for recording
	log_step "Showing final result..."
	sleep 3

	# Exit the app
	log_step "Exiting app..."
	send_keys "C-c" 1
	sleep 1

	# Exit asciinema (sends exit to the shell which asciinema is recording)
	log_step "Stopping recording..."
	send_keys "exit" 0.5
	send_enter 1

	# Wait for asciinema to finish writing and close
	log_step "Waiting for recording to finalize..."
	sleep 5

	# Close the demo pane (it should already be closed, but just in case)
	tmux kill-pane -t "$DEMO_PANE" 2>/dev/null || true

	# Wait a bit more for file system sync
	sleep 2

	echo ""
	echo "✅ Recording complete: docs/demo.cast"
	echo ""
	echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
	echo ""

	# Generate screenshot
	echo "📸 Generating screenshot..."
	if agg --font-family "monospace" --theme monokai --font-size 14 --speed 1 --last-frame-duration 3 docs/demo.cast docs/screenshot.gif; then
		echo "✅ Screenshot generated: docs/screenshot.gif"

		# Convert GIF to PNG if ImageMagick is available
		if command -v convert &>/dev/null; then
			convert 'docs/screenshot.gif[0]' docs/recording.gif 2>/dev/null &&
				echo "✅ Gif: docs/recording.gif"
		fi
	else
		echo "⚠️  Screenshot generation failed"
		echo "You can generate manually with:"
		echo "   agg docs/demo.cast docs/recording.gif"
	fi
	echo ""

	# Upload to asciinema if requested
	CAST_ID=""
	if $UPLOAD; then
		echo "📤 Uploading to asciinema.org..."
		UPLOAD_OUTPUT=$(asciinema upload docs/demo.cast 2>&1)
		echo "$UPLOAD_OUTPUT"

		# Extract cast ID from output
		CAST_ID=$(echo "$UPLOAD_OUTPUT" | grep -oP 'https://asciinema.org/a/\K[a-zA-Z0-9]+' | head -1)

		if [ -z "$CAST_ID" ]; then
			echo ""
			echo "⚠️  Failed to extract cast ID from upload response"
			echo "Continuing without asciinema.org link..."
			echo ""
		else
			echo ""
			echo "✅ Upload successful!"
			echo "   Cast ID: $CAST_ID"
			echo "   URL: https://asciinema.org/a/$CAST_ID"
			echo ""
		fi
	fi

	# Update README.new.md
	echo "📝 Updating README.new.md..."

	# Uncomment the screenshot line if we have a PNG
	if [ -f "docs/screenshot.png" ]; then
		sed -i "s/<!-- \(.*screenshot\.png.*\) -->/\1/" README.new.md
	fi

	# If we uploaded and got a cast ID, use asciinema.org link
	if [ -n "$CAST_ID" ]; then
		sed -i "s/YOUR_CAST_ID/$CAST_ID/g" README.new.md
		sed -i "s/<!-- \(.*asciicast.*\) -->/\1/" README.new.md
	fi

	echo "✅ README.new.md updated"
	echo ""

	# Offer to replace README.md
	echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
	echo ""
	echo "Replace README.md with README.new.md?"
	read -p "(y/N) " -n 1 -r
	echo ""

	if [[ $REPLY =~ ^[Yy]$ ]]; then
		mv README.md README.old.md
		mv README.new.md README.md
		echo "✅ README.md updated (old version saved as README.old.md)"
	else
		echo "ℹ️  Keeping README.new.md (run 'mv README.new.md README.md' to use it)"
	fi

	echo ""
	echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
	echo ""
	echo "🎉 Demo complete!"
	echo ""
	echo "Files created:"
	echo "  ✓ docs/demo.cast"
	if [ -f "docs/screenshot.gif" ]; then
		echo "  ✓ docs/screenshot.gif"
	fi
	if [ -f "docs/screenshot.png" ]; then
		echo "  ✓ docs/screenshot.png"
	fi
	echo "  ✓ README.new.md (updated)"
	echo ""
	if [ -n "$CAST_ID" ]; then
		echo "Cast URL: https://asciinema.org/a/$CAST_ID"
		echo ""
	fi
else
	echo ""
	echo "✅ Demo complete!"
	echo ""
	echo "The app is still running in the demo pane."
	echo "Press Ctrl+C in that pane to exit, or run:"
	echo "   tmux kill-pane -t $DEMO_PANE"
	echo ""
	echo "Timing looks good? Run with --record to create the demo.cast"
	echo "   ./record-demo.sh --record"
fi
