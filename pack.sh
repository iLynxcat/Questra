#!/bin/zsh

# Shitty packing script
# 	Download cross-compiled files from GitHub Actions
#   in this order:
# 		- macOS   - universal	- downloads as    `questra'
# 		- Linux   - x86_64		- auto-renames to `questra-2'
# 		- Windows - x86_64		- downloads as    `questra.exe'
# 	Then run this script:
# 		./pack.sh questra-{version}.zip

local filename=$1
if [[ "$filename" != *.zip ]]; then
  echo "Error: zip filename must end in .zip" >&2
  exit 1
fi

local exec1="$HOME/Downloads/questra"
local exec2="$HOME/Downloads/questra-2"
local execwin="$HOME/Downloads/questra.exe"

if ! file "$exec1" | grep -q "Mach-O"; then
  echo "Error: expected a Mach-O binary" >&2
  exit 1
fi

if ! file "$exec2" | grep -q "GNU/Linux"; then
  echo "Error: expected a Linux binary" >&2
  exit 1
fi

if ! file "$execwin" | grep -q "MS Windows"; then
  echo "Error: expected a Windows binary" >&2
  exit 1
fi

chmod +x $exec1

zip $filename res/**/*
zip $filename -j $exec1
mv $exec2 "$exec1-linux"
zip $filename -j "$exec1-linux"
zip $filename -j $execwin
