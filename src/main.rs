use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use rodio::{Decoder, OutputStream, source::Source};
use std::env;
/// Search for a pattern in a file and display the lines that contain it.

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).unwrap();
    println!("path: {}", path);
    // once we have the path, we need to load the mp3 file
    // Get an output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(path).unwrap());
    println!("file: {:?}", file);
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples());

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(5));
    println!("Where do we start?");
}
