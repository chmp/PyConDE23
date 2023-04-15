use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
    time::Instant,
};

use anyhow::Result;
use serde::Deserialize;
use zip::ZipArchive;

#[allow(unused)]
#[derive(Debug, Clone, Deserialize)]
struct Container {
    playlists: Vec<Playlist>,
}

#[allow(unused)]
#[derive(Default, Debug, Clone, Deserialize)]
struct Playlist {
    name: String,
    collaborative: String,
    pid: i64,
    modified_at: i64,
    num_tracks: u16,
    num_albums: u16,
    num_followers: i64,
    tracks: Vec<Track>,
}

#[allow(unused)]
#[derive(Default, Debug, Clone, Deserialize)]
struct Track {
    pos: u16,
    duration_ms: i64,
    artist_name: String,
    artist_uri: String,
    track_uri: String,
    track_name: String,
    album_uri: String,
    album_name: String,
}

fn main() -> Result<()> {
    const USAGE: &'static str = "Call as json_to_arrow {zip-file}";

    let zip_path = std::env::args_os().nth(1).expect(USAGE);
    let zip_path = PathBuf::from(zip_path);

    println!("Convert data");
    let n = 1_000;
    let start = Instant::now();

    let file = File::open(&zip_path)?;
    let file = BufReader::new(file);
    let mut zip = ZipArchive::new(file)?;

    let mut count = 0;

    for i in 0..n {
        if i % 20 == 0 {
            let took = (Instant::now() - start).as_secs_f32();
            println!("{i} / {n}: {took:.2}");
        }

        let mut content = Vec::new();
        zip.by_name(&format!(
            "data/mpd.slice.{}-{}.json",
            1000 * i,
            1000 * (i + 1) - 1
        ))?
        .read_to_end(&mut content)?;

        let data: Container = serde_json::from_slice(&content)?;
        for playlist in data.playlists {
            count += playlist.tracks.len();
        }
    }

    println!("number of tracks: {count}");

    Ok(())
}
