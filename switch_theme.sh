#!/bin/bash

# Function to create/replace symlinks
update_files() {
  # Update symlinks
  for theme in $(find "$1" -type f -printf '%P\n'); do
    echo "file found: $1/$theme -> $HOME/.config/$theme"
    ln -sf "$1/$theme" $HOME/.config/$theme
  done

  # Set the background
  swww img $2 --transition-fps 60 --transition-type wipe --transition-duration 1

  # Restart waybar
  pkill waybar
  nohup waybar >/dev/null 2>&1 &
}

# Function to switch themes
switch_theme() {
  local theme_name=$1
  local theme_wallpaper=""
  local theme_config_dir=""

  # Find the theme details from the config file
  for i in $(seq 1 $NUM_THEMES); do
    # get variables from config file
    eval current_theme_name=\$THEME${i}_NAME
    eval current_theme_wallpaper=\$WALLPAPER_NAME
    eval current_theme_config_dir=\$THEME${i}_CONFIG_DIR

    if [[ "$current_theme_name" == "$theme_name" ]]; then
      theme_config_dir=$current_theme_config_dir
      theme_wallpaper="${theme_config_dir}/${current_theme_wallpaper}"
      break
    fi
  done

  # if theme does not exist in config
  if [[ -z "$theme_wallpaper" || -z "$theme_config_dir" ]]; then
    echo "Theme '$theme_name' not found in configuration."
    exit 1
  fi

  update_files $theme_config_dir $theme_wallpaper $theme_name
  echo "Updated config files"

  # generate the colors using wal
  wal -i $theme_wallpaper -n -q -t --saturate 0.5 2>/dev/null

  # set the background opacity of kitty
  sed -i '3s/.*/background_opacity 0.6/' ~/.cache/wal/colors-kitty.conf

  # change the fastfetch logo
  magick -size 500x500 xc:none -draw "roundrectangle 0,0,500,500,32,32" ~/.config/fastfetch/mask.png 2>/dev/null
  magick $theme_wallpaper -resize 500x500^ -gravity center -extent 500x500 -alpha set ~/.config/fastfetch/mask.png -compose DstIn -composite /home/mxstoto/.config/fastfetch/fetch-logo.png 2>/dev/null

  echo "Applied wallpaper and colorscheme"
}

# Create config files if they don't exist
if [ -d "$HOME/.config/themeSwitcher" ]; then
  echo ""
else
  echo "Creating themeSwitcher config files..."
  cp -r ./config/themeSwitcher ~/.config/
fi

# Load the configuration file
source ~/.config/themeSwitcher/themes.conf

# Check if any parameter is provided
if [[ $# -eq 0 ]]; then
  echo "Usage: $0 [--list(-l)] | [--select(-s) <theme_name>]"
  exit 1
fi

# Handle command-line parameters
case $1 in
-l | --list)
  if [[ $# -ne 1 ]]; then
    echo "Invalid option: $1"
    echo "Usage: $0 [--list(-l)] | [--select(-s) <theme_name>]"
    exit 1
  fi

  # List all themes
  echo Themes found in config:
  ls -1 ~/Themes/
  ;;
-s | --select | *)
  # Select a specific theme by number
  if [[ $# -ne 1 ]]; then
    echo "Invalid option: $1"
    echo "Usage: $0 [--list(-l)] | [--select(-s) <theme_name>]"
    exit 1
  fi

  switch_theme "$1"
  echo -e "\n> Switched theme to $1"
  ;;
esac
