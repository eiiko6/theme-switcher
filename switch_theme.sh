#!/bin/bash

update_files() {
  # Update symlinks
  ln -sf $1/waybar/config.jsonc ~/.config/waybar/config.jsonc
  ln -sf $1/waybar/style.css ~/.config/waybar/style.css
  ln -sf $1/kitty/kitty.conf ~/.config/kitty/kitty.conf
  ln -sf $1/wofi/style.css ~/.config/wofi/style.css
  ln -sf $1/hypr/appearance.conf ~/.config/hypr/appearance.conf
  ln -sf $1/hypr/animations.conf ~/.config/hypr/animations.conf
  ln -sf $1/random-krabby/pokemon-list.conf ~/.config/random-krabby/pokemon-list.conf
  ln -sf $1/nvim/lua/plugins/colorscheme.lua ~/.config/nvim/lua/plugins/colorscheme.lua

  # Set the background
  swww img $2 --transition-fps 60 --transition-type wipe --transition-duration 1

  # Restart waybar
  pkill waybar
  nohup waybar >/dev/null 2>&1 &
}

# Function to switch themes
switch_theme() {
  local theme_name=$1
  local theme_num=0
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
      theme_num=$i
      break
    fi
  done

  if [[ -z "$theme_wallpaper" || -z "$theme_config_dir" ]]; then
    echo "Theme '$theme_name' not found in configuration."
    exit 1
  fi

  # Update the current theme file with the next theme number
  next_theme_num=$((theme_num % NUM_THEMES + 1))
  echo $next_theme_num >./next_theme_num

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
  echo "> Switched theme to $1"
  ;;
esac
