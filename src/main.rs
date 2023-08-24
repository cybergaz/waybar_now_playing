use serde_json::json;
use std::io::{self, Write};
use waybar_player::PlayerCtl as player;

fn main() -> io::Result<()> {
    let metadata = player::metadata();

    // println!("{:#?}", metadata);
    let mut text = metadata.title + " - " + &metadata.artist;
    // println!("{text}");
    text = text.replace("&", "and");

    if text.len() > 60 {
        text = text[0..60].to_string();
        text = text + ".....";
    }

    let output = json!({ "text": text, "class":"custom-spotify","alt":"spotify" });
    let serialized_output = serde_json::to_string(&output)?;

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    handle.write_all(serialized_output.as_bytes())?;
    handle.write_all(b"\n")?;

    handle.flush()?; // Flush the output to ensure it's written immediately

    Ok(())

    // let ghost = playerctl::PlayerCtl::metadata();
    // println!("{}", ghost);
}
