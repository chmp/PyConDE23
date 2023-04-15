use std::{
    fs::File,
    io::{BufReader, BufWriter, Read},
    path::{Path, PathBuf},
    time::Instant,
};

use anyhow::Result;
use arrow2::{
    chunk::Chunk,
    datatypes::{DataType, Field, Schema},
};
use serde::{Deserialize, Serialize};
use serde_arrow::{
    arrow2::{experimental::find_field_mut, serialize_into_fields, ArraysBuilder},
    schema::TracingOptions,
};
use zip::ZipArchive;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Container {
    playlists: Vec<Playlist>,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
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

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
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
    const USAGE: &'static str = "Call as convert-spotify-playlists {zip-file} {out-path}";

    let zip_path = std::env::args_os().nth(1).expect(USAGE);
    let zip_path = PathBuf::from(zip_path);

    let out_path = std::env::args_os().nth(2).expect(USAGE);
    let out_path = PathBuf::from(out_path);

    let mut fields = serialize_into_fields(
        &[Playlist {
            tracks: vec![Track::default()],
            ..Playlist::default()
        }],
        TracingOptions::default().string_dictionary_encoding(true),
    )?;

    // interpret the modified_at field as a timestamp
    *find_field_mut(&mut fields, "modified_at")? =
        Field::new("modified_at", DataType::Date64, false);

    println!("Convert data");
    let n = 1_000;
    let start = Instant::now();

    let file = File::open(zip_path)?;
    let file = BufReader::new(file);
    let mut zip = ZipArchive::new(file)?;

    let mut builder = ArraysBuilder::new(&fields)?;
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

        let mut data: Container = serde_json::from_slice(&content)?;
        for item in data.playlists.iter_mut() {
            item.modified_at = 1000 * item.modified_at;
        }

        builder.extend(&data.playlists)?;
    }

    let (mut writer, ipc_fields) = build_ipc_writer(out_path, &fields)?;
    let arrays = builder.build_arrays()?;
    writer.write(&Chunk::try_new(arrays)?, Some(&ipc_fields))?;
    writer.finish()?;

    Ok(())
}

fn build_ipc_writer<P: AsRef<Path>>(
    path: P,
    fields: &[Field],
) -> Result<(
    arrow2::io::ipc::write::FileWriter<BufWriter<File>>,
    Vec<arrow2::io::ipc::IpcField>,
)> {
    use arrow2::io::ipc::write::{self, FileWriter, WriteOptions};

    let file = File::create(path)?;
    let file = BufWriter::new(file);

    let schema = Schema::from(fields.to_vec());
    let ipc_fields = write::default_ipc_fields(&schema.fields);

    let options = WriteOptions { compression: None };
    let writer = FileWriter::try_new(file, schema, Some(ipc_fields.clone()), options)?;

    Ok((writer, ipc_fields))
}
