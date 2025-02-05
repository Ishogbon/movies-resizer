use clap::Parser;
use std::{fs, process::Command};

#[derive(Parser)]
struct Args {
    source: String,
    destination: String,
}

fn process_video(input: &str, output: &str) {
    let status = Command::new("ffmpeg")
        .args(["-i", input])
        .args(["-vf", "scale=540:-1,fps=1"])
        .args(["-t", "180"])
        // .args(["-c:v", "libx264", "-preset", "fast", "-crf", "23"])
        // .args(["-c:a", "aac", "-b:a", "128k", output])
        .args(["-f", "rawvideo", "-pix_fmt", "rgb24", output])
        .status()
        .expect("Failed to execute ffmpeg");

    if status.success() {
        println!("Processed: {} -> {}", input, output);
    } else {
        eprintln!("Failed to process: {}", input);
    }
}

fn main() {
    let args = Args::parse();
    let files: Vec<_> = fs::read_dir(args.source)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .collect();

    files.iter().for_each(|file| {
        let mut destination = args.destination.clone();
        destination.push('/');
        destination.push_str(file.file_name().to_str().unwrap());
        process_video(file.path().to_str().unwrap(), destination.as_str());
    });
}
