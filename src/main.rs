use clap::Parser;
use std::{
    fs::{self, File},
    io::{BufReader, Read},
    process::Command,
};

#[derive(Parser)]
struct Args {
    source: String,
    destination: String,
}

fn get_video_dimensions(file_path: &str) -> (u32, u32) {
    let output = Command::new("ffprobe")
        .arg("-v")
        .arg("error")
        .arg("-select_streams")
        .arg("v:0")
        .arg("-show_entries")
        .arg("stream=width,height")
        .arg("-of")
        .arg("default=noprint_wrappers=1:nokey=1")
        .arg(file_path)
        .output()
        .expect("Failed to query Video Dimensions");

    if !output.status.success() {
        panic!("Command failed to query video dimensions");
    }

    let output_str =
        String::from_utf8(output.stdout).expect("Unable to parse string from dimensions std out");
    let results: Vec<&str> = output_str.split_whitespace().collect();

    if results.len() == 2 {
        let width = results[0].parse::<u32>().unwrap();
        let height = results[1].parse::<u32>().unwrap();
        return (width, height);
    }
    panic!("Something went wrong parsing dimensions");
}

fn read_buffer(file_path: &str, width: u16, height: u16) {
    let scale: usize = width as usize * height as usize * 3;

    let file = File::open(file_path).expect("Unable to read file");
    let mut reader = BufReader::new(file);

    let mut buffer = vec![0u8; scale];

    let mut frame_number = 0;

    while reader.read_exact(&mut buffer).is_ok() {
        frame_number += 1;

        // Example: Get the top-left pixel of the frame
        let r = buffer[0];
        let g = buffer[1];
        let b = buffer[2];

        println!(
            "Frame {} - First pixel R: {}, G: {}, B: {}",
            frame_number, r, g, b
        );
    }

    println!("Finished reading {} frames", frame_number);
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
        read_buffer(&destination);
    });
}
