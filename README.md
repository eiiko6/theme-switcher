## Introduction

**dotswitch** (dotfiles switcher) basically creates symlinks from a profile directory for your config files.
I tried making it customizable.
Each profile can have a different script if specific actions are required.

## Usage

- Place your configuration files in a directory in the path specified in the config.
- Run `dotswitch switch <profile_name> <profile_name>` to enable a profile. You can also list your profiles with `dotswitch list`.

> Warning: this **will** overwrite the files currently present in **.config**, replacing them with symlinks to the corresponding files in the selected profile. Use the `backup` option to backup you files, and/or the `preview` option to preview changes.

## Configuration

The script will read `~/.config/dotswitch/dotswitch.conf`.

Specify the location of your profiles (required):
> `profiles_dir = ~/some-path/`

You can add multiple global scripts executed at each profile switch with:
> `global_script = ~/some-path/some-script.sh`
> `global_script = ~/some-path/some-other-script.sh`

Each profile can have scripts at its root:
> `per_profile_script = script.sh`
> `per_profile_script = some-other-script.sh`
> (relative to the profile's location)

-> See my [dotfiles](https://github.com/eiiko6/dotfiles) for an example on how to integrate this tool.

## Modules

Each profile has this structure:

```txt
profile-name/
  ├── module1/
  │   ├── hypr/
  │   ├── kitty/
  │   ├── mako/
  │   └── nvim/
  ├── module2/
  │   └── waybar/
  └── module3/
      └── waybar/
```

where the modules (with whatever names) can for example contain machine-specific files, that could differ between laptop and desktop.
