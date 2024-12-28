#!/bin/bash

# Function to create/replace symlinks
update_files() {
  local theme_dir=$1
  for theme in $(find "$theme_dir" -type f -printf '%P\n'); do
    ln -sf "$theme_dir/$theme" "$HOME/.config/$theme"

    if [[ "$quiet" != 1 ]]; then
      echo "Created symlink: $theme_dir/$theme -> $HOME/.config/$theme"
    fi
  done
}

# Function to execute global commands
execute_global_commands() {
  local wallpaper="$1"
  local theme_name="$2"

  # Iterate over each global command
  while IFS= read -r command; do
    # Skip empty lines
    [[ -z "$command" ]] && continue

    # Replace placeholders and execute
    local expanded_command=$(echo "$command" | sed \
      -e "s|\$wallpaper|$wallpaper|g" \
      -e "s|\$theme_name|$theme_name|g")

    echo "Executing: $expanded_command"

    eval "$expanded_command"
  done <<<"$GLOBAL_COMMANDS"
}

# Function to switch themes
switch_theme() {
  local theme_name=$1
  local theme_wallpaper=""
  local theme_config_dir=""

  # Automatically detect theme directories
  local theme_dir="$HOME/.config/theme-switcher/themes/$theme_name"

  if [[ ! -d "$theme_dir" ]]; then
    echo "Theme '$theme_name' not found in ~/.config/theme-switcher/themes."
    exit 1
  fi

  # Look for the wallpaper and config details in the theme directory
  theme_wallpaper="$theme_dir/wallpaper.png"
  theme_config_dir="$theme_dir"

  update_files "$theme_config_dir"
  echo "=> Updated config files"
  echo ""

  execute_global_commands "$theme_wallpaper" "$theme_name"
  echo "=> Executed global commands."
  echo ""
}

# Create default config directory if not present
if [[ ! -d "$HOME/.config/theme-switcher" ]]; then
  echo "Creating theme-switcher config files..."
  mkdir -p "$HOME/.config/theme-switcher"
  cp -r ./config/theme-switcher/* "$HOME/.config/theme-switcher/"
fi

# Load themes.conf
source "$HOME/.config/theme-switcher/themes.conf"

# Create themes directory if it doesn't exist
if [[ ! -d "$HOME/.config/theme-switcher/themes" ]]; then
  echo "Creating themes directory..."
  mkdir -p "$HOME/.config/theme-switcher/themes"
fi

# List themes if requested
if [[ $# -eq 0 ]]; then
  echo "Usage: $0 [--list(-l)] | [--select(-s) <theme_name>]"
  exit 1
fi

# Handle parameters
while [[ $# -gt 0 ]]; do
  case $1 in
  -l | --list)
    echo "Themes found in ~/.config/theme-switcher/themes:"
    ls -1 ~/.config/theme-switcher/themes/
    ;;
  -q | --quiet)
    quiet=1
    ;;
  *)
    switch_theme "$1"
    ;;
  esac
  shift
done
