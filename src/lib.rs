//! # Playerctl
use std::{process::Command, str::FromStr};

/// Runs a command and returns the output.
///
/// # Panics
///
/// Panics if the command fails to execute.
fn command(command: &str) -> String {
    let mut parts = command.split_whitespace().collect::<Vec<&str>>();
    let stdout = Command::new(parts.remove(0))
        .args(parts)
        .output()
        .unwrap_or_else(|_| panic!("Failed to execute command '{}'", command))
        .stdout;

    String::from_utf8(stdout).expect("Stdout was not valid UTF-8")
}

/// The current state of the player.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerStatus {
    /// Media is currently playing.
    Playing,
    /// Media is currently paused.
    Paused,
    /// Media is currently stopped.
    Stopped,

    NoPlayer,
}

impl FromStr for PlayerStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Playing" => Ok(Self::Playing),
            "Paused" => Ok(Self::Paused),
            "Stopped" => Ok(Self::Stopped),
            _ => Err(()),
        }
    }
}

/// The player's looping state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopStatus {
    /// Media is not looping.
    None,
    /// The current track will loop.
    Track,
    /// All tracks will loop.
    Playlist,
}

impl FromStr for LoopStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "None" => Ok(Self::None),
            "Track" => Ok(Self::Track),
            "Playlist" => Ok(Self::Playlist),
            _ => Err(()),
        }
    }
}

/// The player's shuffle state to set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShuffleStatus {
    /// Media will be shuffled.
    On,
    /// Media will not be shuffled.
    Off,
    /// The shuffle status will be toggled.
    Toggle,
}

impl FromStr for ShuffleStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "On" => Ok(Self::On),
            "Off" => Ok(Self::Off),
            "Toggle" => Ok(Self::Toggle),
            _ => Err(()),
        }
    }
}

/// The currently playing track's metadata.
///
/// If any of the metadata is not available, that field will be an empty string.
#[derive(Debug)]
pub struct TrackMetadata {
    /// The track's artist.
    pub artist: String,
    /// The track's title.
    pub title: String,
    /// The track's album.
    pub album: String,
}

/// The playerctl library.
///
/// Allows you to control the player.
pub struct PlayerCtl;

impl PlayerCtl {
    /// Play media.
    pub fn play() {
        command("playerctl play");
    }

    /// Pause media.
    pub fn pause() {
        command("playerctl pause");
    }

    /// Play / pause media.
    pub fn play_pause(active_player: &Option<String>) {
        if let Some(active_player) = active_player {
            command(&format!("playerctl -p {} play-pause", active_player));
        } else {
            command("playerctl play-pause");
        }
    }

    /// Stop media.
    pub fn stop() {
        command("playerctl stop");
    }

    /// Skip to the next track.
    pub fn next(active_player: &Option<String>) {
        if let Some(active_player) = active_player {
            command(&format!("playerctl -p {} next", active_player));
        } else {
            command("playerctl next");
        }
    }

    /// Skip to the previous track.
    pub fn previous(active_player: &Option<String>) {
        if let Some(active_player) = active_player {
            command(&format!("playerctl -p {} previous", active_player));
        } else {
            command("playerctl next");
        }
    }

    /// Seek forwards / backwards in seconds.
    pub fn position(secs: f32) {
        if secs < 0. {
            command(&format!("playerctl position {}-", -secs));
        } else {
            command(&format!("playerctl position {}+", secs));
        }
    }

    /// Set the volume to the given percentage from 0 to 1.
    pub fn volume(percent: f32) {
        if percent < 0. {
            command(&format!("playerctl volume {}-", -percent));
        } else {
            command(&format!("playerctl volume {}+", percent));
        }
    }

    /// Gets the current player status.
    ///
    /// # Panics
    ///
    /// Will panic if playerctl returns an invalid status.
    #[must_use]
    pub fn status() -> PlayerStatus {
        let status_str = &command("playerctl status").trim().to_owned()[..];

        match status_str {
            "Playing" | "Paused" | "Stopped" => {
                status_str.parse().unwrap_or(PlayerStatus::NoPlayer)
            }
            "No players found" => PlayerStatus::NoPlayer,
            _ => PlayerStatus::NoPlayer, // Handle any other unexpected cases
        }
    }

    /// Get the metadata of the currently playing track.
    #[must_use]
    pub fn metadata(active_player: &Option<String>) -> TrackMetadata {
        let mut title = command("playerctl metadata title").trim().to_string();
        let mut artist = command("playerctl metadata artist").trim().to_string();
        let mut album = command("playerctl metadata album").trim().to_string();
        if let Some(active_player) = active_player {
            title = command(&format!("playerctl -p {} metadata title", active_player))
                .trim()
                .to_string();
            artist = command(&format!("playerctl -p {} metadata artist", active_player))
                .trim()
                .to_string();
            album = command(&format!("playerctl -p {} metadata album", active_player))
                .trim()
                .to_string();
        }

        TrackMetadata {
            artist,
            title,
            album,
        }
    }

    /// Open the given uri in the player.
    ///
    /// The uri can be a file path or web url.
    pub fn open(uri: &str) {
        command(&format!("playerctl open {uri}"));
    }

    /// Get the current loop status.
    ///
    /// # Panics
    ///
    /// Will panic if playerctl returns an invalid status.
    #[must_use]
    pub fn loop_get() -> LoopStatus {
        command("playerctl loop")
            .trim()
            .parse()
            .expect("Failed to parse loop status")
    }

    /// Set the loop status.
    pub fn loop_set(status: LoopStatus) {
        command(&format!("playerctl loop {:?}", status));
    }

    /// Get the current shuffle status.
    ///
    /// # Panics
    ///
    /// Will panic if playerctl returns an invalid status.
    #[must_use]
    pub fn shuffle_get() -> ShuffleStatus {
        command("playerctl shuffle")
            .trim()
            .parse()
            .expect("Failed to parse shuffle status")
    }

    /// Set the shuffle status.
    pub fn shuffle_set(status: ShuffleStatus) {
        command(&format!("playerctl shuffle {:?}", status));
    }

    /// list all the available players
    pub fn list_all() -> Vec<String> {
        command("playerctl -l")
            .trim()
            .split('\n')
            .map(String::from)
            .collect()
    }

    /// status for given player
    pub fn status_of(player: &str) -> PlayerStatus {
        let status_str = &command(&format!("playerctl -p {} status", player))
            .trim()
            .to_owned()[..];

        match status_str {
            "Playing" | "Paused" | "Stopped" => {
                status_str.parse().unwrap_or(PlayerStatus::NoPlayer)
            }
            "No players found" => PlayerStatus::NoPlayer,
            _ => PlayerStatus::NoPlayer, // Handle any other unexpected cases
        }
    }
}
