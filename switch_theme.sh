#!/bin/bash

# Load the configuration file
source ./themes.conf

# Function to switch themes
switch_theme() {
    local theme_name=$1
    local theme_wallpaper=""
    local theme_config_dir=""

    # Find the theme details from the config file
    for i in $(seq 1 $NUM_THEMES); {
        eval current_theme_name=\$THEME${i}_NAME
        eval current_theme_wallpaper=\$THEME${i}_WALLPAPER
        eval current_theme_config_dir=\$THEME${i}_CONFIG_DIR

        if [[ "$current_theme_name" == "$theme_name" ]]; then
            theme_wallpaper=$current_theme_wallpaper
            theme_config_dir=$current_theme_config_dir
            break
        fi
    }

    if [[ -z "$theme_wallpaper" || -z "$theme_config_dir" ]]; then
        echo "Theme '$theme_name' not found in configuration."
        exit 1
    fi

    # Update symlinks
    ln -sf $theme_config_dir/waybar/config.jsonc ~/.config/waybar/config.jsonc
    ln -sf $theme_config_dir/waybar/style.css ~/.config/waybar/style.css
    # ln -sf $theme_config_dir/wofi/style.css ~/.config/wofi/style.css

    # Set the background
    swww img $theme_wallpaper --transition-fps 60 --transition-type wipe --transition-duration 1

    # Update the current theme file
    echo $theme_name > ./current_theme

    # Restart waybar
    pkill waybar
    nohup waybar &
}

# Check if the theme name is provided
if [[ -z "$1" ]]; then
    echo "Usage: $0 <theme_name>"
    exit 1
fi

# Switch to the specified theme
switch_theme $1

