use serde_json::json;
use std::env;
use std::io::{self, Write};
use waybar_now_playing::PlayerCtl;
use waybar_now_playing::PlayerStatus as ps;

fn draw(active_player: &Option<String>) -> io::Result<()> {
    let mut metadata = PlayerCtl::metadata(&None);
    if active_player.is_some() {
        metadata = PlayerCtl::metadata(&active_player);
    }

    let mut text = metadata.title + " - " + &metadata.artist;
    // println!("{text}");
    text = text.replace("&", "and");

    if text.len() > 60 {
        text = text[0..60].to_string();
        text = text + ".....";
    }

    let output = json!({ "text": text, "class":"custom-player","alt":"playing" });
    let serialized_output = serde_json::to_string(&output)?;
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    handle.write_all(serialized_output.as_bytes())?;
    handle.write_all(b"\n")?;

    handle.flush()?; // Flush the output to ensure it's written immediately

    Ok(())
}

fn vanish() -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(b"\n")?;
    handle.flush()?;

    Ok(())
}

fn main() {
    match PlayerCtl::status() {
        ps::NoPlayer => vanish().unwrap(),

        _ => {
            let active_player = active_player();
            if active_player.is_some() {
                // throws json data to std output with intended player
                draw(&active_player).unwrap();

                let args: Vec<String> = env::args().collect();

                // check if player name is on disk in case there is no active_player
                if std::path::Path::new("/tmp/waybar_player").exists() && active_player.is_none() {
                    let player = Some(std::fs::read_to_string("/tmp/waybar_player").unwrap());
                    handle_args(&player, args)
                } else {
                    handle_args(&active_player, args);
                }
            } else {
                // runs when there is nothing playing
                // throws json data to std output without any intended player
                draw(&None).unwrap();

                let args: Vec<String> = env::args().collect();
                // so if there is no active player , check if there is some content written on the disk , if there is.. it'll work as a resume function
                if std::path::Path::new("/tmp/waybar_player").exists() {
                    let player = Some(std::fs::read_to_string("/tmp/waybar_player").unwrap());
                    handle_args(&player, args)
                }
            }
        }
    }
}

fn active_player() -> Option<String> {
    let all_players = PlayerCtl::list_all();

    for p in all_players {
        if PlayerCtl::status_of(&p) == ps::Playing {
            return Some(p);
        }
    }
    None
}

fn handle_args(active_player: &Option<String>, args: Vec<String>) {
    if active_player.is_some() {
        if let Some(input) = args.get(1) {
            if *input == "play-pause".to_string() {
                PlayerCtl::play_pause(&active_player);
                std::fs::write(
                    "/tmp/waybar_player",
                    format!("{}", active_player.as_ref().unwrap()),
                )
                .unwrap();
            } else if *input == "next".to_string() {
                PlayerCtl::next(&active_player);
                std::fs::write(
                    "/tmp/waybar_player",
                    format!("{}", active_player.as_ref().unwrap()),
                )
                .unwrap();
            } else if *input == "previous".to_string() {
                PlayerCtl::previous(&active_player);
                std::fs::write(
                    "/tmp/waybar_player",
                    format!("{}", active_player.as_ref().unwrap()),
                )
                .unwrap();
            }
        }
    }
}
