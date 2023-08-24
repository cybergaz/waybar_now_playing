<div align="center">

  waybar player
  =============

  currently playing media representation on waybar written in rust which provide nearly zero performance and memory cost for updating even at the interval of 1 second


</div>

### dependencies
you need to have `playerctl` installed
```
sudo pacman -S playerctl
```
  
### manual build

```
git clone https://github.com/cybergaz/waybar_player_rust
cd waybar_player_rust
cargo build
```

<br>

### integration with waybar 
just update the waybar config by adding new module , for example :
```
  "custom/player": {
    "exec": " path/to/binary ",
    "format": "Playing :    {}",
    "interval": 2,
    "return-type": "json",
    "on-click": "playerctl play-pause"
  },
```
