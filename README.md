<div align="center">

  waybar now-playing
  =============

  current track/song/media visualization on waybar written in rust which provide near zero performance and memory cost for updating even at the interval of 1 second


</div>

### dependencies
you need to have `playerctl` installed
```
sudo pacman -S playerctl
```

### get the binary here 
* [download binary](www.github.com)

### manual build

```
git clone https://github.com/cybergaz/waybar_now_playing
cd waybar_now_playing
cargo build
```

<br>

### integration in waybar 
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
