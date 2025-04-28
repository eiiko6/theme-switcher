## Introduction

This script basically creates symlinks from a theme directory for your config files.
I tried making it customizable through a config file.
Each theme can have a different script if specific actions are required.

## Usage

- Copy your configuration files in a directory in the path specified in the config. The wallpaper should be in the root of the config's directory.
- Run `./switch_theme.sh <theme_name>` to enable a theme. You can also list your themes with `./switch-theme.sh -l`.

> Warning: this **will** overwrite the files currently present in **.config**, replacing them with symlinks to the corresponding files in the selected theme. Use the `-b` option to backup you files.

## Themes

Each theme has this structure:

```txt
theme-name/
  ├── common/
  │   ├── hypr/
  │   ├── kitty/
  │   ├── mako/
  │   └── nvim/
  ├── desktop/
  │   └── waybar/
  └── laptop/
      └── waybar/
```

where desktop and laptop contain machine-specific files, that could differ between laptops and desktops.

## Configuration

The script will read `~/.config/theme-switcher/themes.conf` to determine where the themes are located.

You can add a global script executed at each theme switch in `~/.config/theme-switcher/script.sh`.

Each theme can have a `script.sh` at its root, as well as a `wallpaper.png` if you want to use it in either the global script or the theme's script.

You can add a function/alias tour your shell for convenience:

```
alias theme='~/Desktop/git-repos/themeSwitcher/switch_theme.sh'
```

> See my [dotfiles](https://github.com/eiiko6/dotfiles) for an example on how to integrate this tool.
