i#!/bin/bash

# --- Ubuntu Terminal Setup Script (Tmux & Fish) ---
# Installs tmux, fish, and htop. Configures tmux for better ergonomics,
# and sets fish as the default shell for the current user.
#
# NOTE: This script requires 'sudo' privileges. but DO NOT `sudo ./this-script.sh`

# Stop the script immediately if any command fails
set -e

# --- 1. System Update and Package Installation ---
echo "--- 1. Installing packages: tmux, fish, htop ---"
sudo apt update
sudo apt install -y tmux fish htop

# Check if packages were installed successfully
if ! command -v tmux &> /dev/null || ! command -v fish &> /dev/null || ! command -v htop &> /dev/null; then
    echo "ERROR: One or more package installations failed. Please check the output above."
    exit 1
fi

# --- 2. Tmux Configuration (~/.tmux.conf) ---
echo "--- 2. Configuring Tmux for mouse and instant response ---"

# Use a here-document to write the configuration file cleanly
cat << EOF > ~/.tmux.conf
# --- Minimal Tmux Ergonomic Settings ---

# Set escape-time to 0ms for instantaneous response (solves key delay issues)
set -g escape-time 0

# Enable mouse support for scrolling and switching panes/windows
set -g mouse on
EOF

echo "Tmux config written."

# --- 3. Change Default Shell ---
echo "--- 3. Changing default shell to Fish for user ($USER) ---"

# Get the full path to the fish executable
FISH_PATH=$(which fish)

# Check if the path was found
if [ -z "$FISH_PATH" ]; then
    echo "ERROR: Could not find the 'fish' executable path."
    exit 1
fi

# Use chsh to change the shell. Requires user password.
echo "Running 'chsh -s $FISH_PATH'. This may prompt for your user password."
sudo chsh -s "$FISH_PATH" "$USER"

echo "--------------------------------------------------------"
echo "✅ Setup Complete: tmux, fish, and htop installed."
