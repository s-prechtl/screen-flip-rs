# screen-flip-rs
Flips the screen for my hyprland config.

## How it works.

I have a variable in a config file for hyprland called `transform.conf` with a  single line.
```
$transform=0
```
The script will get the number and switch it to 2, or if 2 to 0.
This file is then sourced at the top of my config and the variable can then be used for screen orientation.
