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

  # Find the theme details from the config file
  for i in $(seq 1 $NUM_THEMES); do
    eval current_theme_name=\$THEME${i}_NAME
    eval current_theme_wallpaper=\$WALLPAPER_NAME
    eval current_theme_config_dir=\$THEME${i}_CONFIG_DIR

    if [[ "$current_theme_name" == "$theme_name" ]]; then
      theme_config_dir=$current_theme_config_dir
      theme_wallpaper="${theme_config_dir}/${current_theme_wallpaper}"
      break
    fi
  done

  if [[ -z "$theme_wallpaper" || -z "$theme_config_dir" ]]; then
    echo "Theme '$theme_name' not found in configuration."
    exit 1
  fi

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

# Load the configuration file
source "$HOME/.config/theme-switcher/themes.conf"

if [[ $# -eq 0 ]]; then
  echo "Usage: $0 [--list(-l)] | [--select(-s) <theme_name>]"
  exit 1
fi

while [[ $# -gt 0 ]]; do
  case $1 in
  -l | --list)
    echo "Themes found in config:"
    ls -1 ~/Themes/
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
