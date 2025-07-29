#!/bin/bash

# Exit immediately on error
set -e

# Input and output filenames
TEMPLATE_FILE="ngrok-bot.service.tmpl"
OUTPUT_FILE="ngrok-bot.service"

# Get current user and current working directory
USER_NAME="$(whoami)"
PROJECT_DIR="$(pwd)"

# Check template file exists
if [ ! -f "$TEMPLATE_FILE" ]; then
    echo "Error: Template file '$TEMPLATE_FILE' not found."
    exit 1
fi

# Replace placeholders and output to new file
sed \
  -e "s|{{ USER }}|$USER_NAME|g" \
  -e "s|{{ PROJECT_DIR }}|$PROJECT_DIR|g" \
  "$TEMPLATE_FILE" > "$OUTPUT_FILE"

echo "Generated $OUTPUT_FILE successfully."

sudo mv ngrok-bot.service /etc/systemd/system/
sudo systemctl enable ngrok-bot.service
sudo systemctl start ngrok-bot.service
