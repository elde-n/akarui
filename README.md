# akarui
A lightweight `/sys/class/backlight/*` wrapper, to control the backlight of screens on linux systems

# installation
To get write access to `/sys/class/backlight/*`
- Install the `contrib/90-backlight.rules` to `/etc/udev/rules.d`
- Or write a init-system script that gives write permissions to your user on boot

```sh
cargo install akarui
```

# usage
```sh
# the default device is the first device found
akarui get --device="intel_backlight" # returns the backlight percentage of the default/selected device
akarui set 100 # sets the backlight percentage of a device to 100
akarui list # lists all the available devices
akarui load # loads all the screen backlight levels from the config file

akarui increase 5 # increases the backlight level by 5%
```
