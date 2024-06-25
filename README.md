## Introduction
This script basically creates symlinks from a **~/Themes/** directory for your config files.
I tried making it customizable through a config file.


## Usage
- Copy your configuration files in **~/Themes/<name_of_the_theme>**, then add their path to **themes.conf**. The wallpaper should be in the root of the config's directory.
- Run `./switch-theme.sh -s <name_of_the_theme>` or `./switch-theme.sh -c` to cycle through your themes. You can also list your themes with `./switch-theme.sh -c`


You can add a function/alias tour your shell:
- For bash, add the following to your **.bashrc**:
```
themeswitcher() {
	path=$(pwd)
	cd ~/Desktop/git-repos/themeSwitcher/
	./switch_theme.sh $1 $2
	cd $path
}

alias theme='themeswitcher'
```
- For fish add the following to your **config.fish**:
```
function themeswitcher
    set path $(pwd)
    cd ~/Desktop/git-repos/themeSwitcher/
    ./switch_theme.sh $argv[1] $argv[2]
    cd $path
end

alias theme='themeswitcher'
```

