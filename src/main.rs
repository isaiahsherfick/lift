use audiotags::{MimeType, Picture, Tag};
use rodio::{source::Source, Decoder, OutputStream};
use std::env;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Error;
use std::thread;
use std::time::Duration;
use tui::{backend::CrosstermBackend, Terminal};
/// Search for a pattern in a file and display the lines that contain it.

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).unwrap();
    let start_time = args.get(2).unwrap().parse::<u64>().unwrap();
    let end_time = args.get(3).unwrap().parse::<u64>().unwrap();
    let speed = args.get(4).unwrap().parse::<f32>().unwrap();
    println!("speed: {}", speed);
    // once we have the path, we need to load the mp3 file
    // Get an output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(path).unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    let total_play_time = end_time - start_time;
    let starter_clone = start_time.clone();
    std::thread::spawn(move || {
        for i in starter_clone..214748 {
            println!("{}", i);
            std::thread::sleep(Duration::from_millis((1000.0 / speed) as u64));
        }
    });
    let selection = source
        .convert_samples::<f32>()
        .skip_duration(Duration::from_millis(start_time * 1000))
        .take_duration(Duration::from_millis(
            ((total_play_time * 1000) as f32 * (1.0 / speed)) as u64,
        ))
        .speed(speed);
    // Play the sound directly on the device
    let _ = stream_handle.play_raw(selection);

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_millis(
        (total_play_time as f32 * 1000.0 / speed) as u64,
    ));
    println!("What a good start!");
    // given source, ask for times within length of song

    // let stdout = io::stdout();
    // let backend = CrosstermBackend::new(stdout);
    // let mut terminal = Terminal::new(backend)?;
    // let mut terminal = Terminal::new(backend)?;
    // Ok(())
}
