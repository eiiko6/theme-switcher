#!/bin/bash

update_files() {
	# Update symlinks
	ln -sf $1/waybar/config.jsonc ~/.config/waybar/config.jsonc
	ln -sf $1/waybar/style.css ~/.config/waybar/style.css
	ln -sf $1/kitty/kitty.conf ~/.config/kitty/kitty.conf
	ln -sf $1/wofi/style.css ~/.config/wofi/style.css
	ln -sf $1/hypr/appearance.conf ~/.config/hypr/appearance.conf
	ln -sf $1/random-krabby/pokemon-list.conf ~/.config/random-krabby/pokemon-list.conf
	ln -sf $1/nvim/lua/plugins/colorscheme.lua ~/.config/nvim/lua/plugins/colorscheme.lua

	# Backup everything, this is kinda temporary
	cp ~/.config/waybar/config.jsonc ./backups/waybar/config.jsonc
	cp ~/.config/waybar/style.css ./backups/waybar/style.css
	cp ~/.config/kitty/kitty.conf ./backups/kitty/kitty.conf
	cp ~/.config/hypr/appearance.conf ./backups/hypr/appearance.conf
	cp ~/.config/random-krabby/pokemon-list.conf ./backups/random-krabby/pokemon-list.conf
	cp ~/.config/nvim/lua/plugins/colorscheme.lua ./backups/nvim/lua/plugins/colorscheme.lua

	# Set the background
	swww img $2 --transition-fps 60 --transition-type wipe --transition-duration 1

	# Restart waybar
	pkill waybar
	nohup waybar &
	echo "> Switched theme to $1"
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
			echo $theme_wallpaper
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
}

# Function to switch themes
switch_theme_num() {
	local theme_num=$1
	local theme_wallpaper=""
	local theme_config_dir=""

	# Find the theme details from the config file
	eval current_theme_name=\$THEME${theme_num}_NAME
	eval current_theme_wallpaper=\$WALLPAPER_NAME
	eval current_theme_config_dir=\$THEME${theme_num}_CONFIG_DIR

	if [[ -z "$current_theme_name" || -z "$current_theme_wallpaper" || -z "$current_theme_config_dir" ]]; then
		echo "Theme '$theme_num' not found in configuration."
		exit 1
	fi

	theme_config_dir=$current_theme_config_dir
	theme_wallpaper="${current_theme_config_dir}/${current_theme_wallpaper}"
	echo $theme_wallpaper

	# Update the current theme file with the next theme number
	next_theme_num=$((theme_num % NUM_THEMES + 1))
	echo $next_theme_num >./next_theme_num

	update_files $theme_config_dir $theme_wallpaper $theme_name
}

# Load the configuration file
source ./themes.conf

# Check if any parameter is provided
if [[ $# -eq 0 ]]; then
	echo "Usage: $0 [--list(-l)] | [--cycle(-c)] | [--select(-s) <theme_name>]"
	exit 1
fi

# Handle command-line parameters
case $1 in
-c | --cycle)
	# Get the current theme number from the file
	next_theme_num=$(<./next_theme_num)
	switch_theme_num "$next_theme_num"
	;;
-s | --select)
	# Select a specific theme by number
	if [[ $# -ne 2 ]]; then
		echo "Usage: $0 -s|--select <theme_name>"
		exit 1
	fi
	switch_theme "$2"
	;;
-l | --list)
	# List all themes
	ls ~/Themes/
	;;
*)
	echo "Invalid option: $1"
	echo "Usage: $0 [--list(-l)] | [--cycle(-c)] | [--select(-s) <theme_name>]"
	exit 1
	;;
esac
