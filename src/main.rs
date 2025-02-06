use clap::Parser;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufReader, Read, Write},
    process::Command,
};

#[derive(Parser)]
struct Args {
    source: String,
    destination: String,
}

fn get_video_dimensions(file_path: &str) -> (u16, u16) {
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
        let width = results[0].parse::<u16>().unwrap();
        let height = results[1].parse::<u16>().unwrap();
        return (width, height);
    }
    panic!("Something went wrong parsing dimensions");
}

fn read_buffer(file_path: &str, width: u16, height: u16) {
    println!("{}, {}, {}", file_path, width, height);
    let scale: usize = width as usize * height as usize * 3;

    let file = File::open(file_path).expect("Unable to read file");
    let mut reader = BufReader::new(file);

    let mut buffer = vec![0u8; scale];

    let mut frame_number = 1;

    let mut frames: HashMap<String, Vec<(u8, u8, u8)>> = HashMap::new();
    while reader.read_exact(&mut buffer).is_ok() {
        let frame_entry = frames
            .entry(format!("frame_{}", frame_number))
            .or_insert_with(Vec::new);

        for pix_position_offset in (0..buffer.len()).step_by(3) {
            if pix_position_offset + 2 < buffer.len() {
                let r = buffer[pix_position_offset];
                let g = buffer[pix_position_offset + 1];
                let b = buffer[pix_position_offset + 2];

                frame_entry.push((r, g, b));

                // println!(
                //     "Frame {} - Pixel R: {}, G: {}, B: {}",
                //     frame_number, r, g, b
                // );
            }
        }
        frame_number += 1;
        println!("Finished reading {} frames", frame_number);
    }
    let frames_json = serde_json::to_string(&frames).expect("Failed to serialize");
    let mut file = File::create(format!("{}{}", file_path, ".json"))
        .expect("Failed to create json file to store frames");
    file.write_all(frames_json.as_bytes())
        .expect("Faile to write frames_json into file");
    println!("Finished writing to {}", file_path);
}

fn process_video(input: &str, output: &str) -> (u16, u16) {
    let (movie_width, movie_height) = get_video_dimensions(input);
    let status = Command::new("ffmpeg")
        .args(["-i", input])
        .args(["-vf", "scale=540:-1,fps=1"])
        .args(["-t", "180"])
        // .args(["-c:v", "libx264", "-preset", "fast", "-crf", "23"])
        // .args(["-c:a", "aac", "-b:a", "128k", output])
        .args(["-f", "rawvideo", "-pix_fmt", "rgb24", output])
        .status()
        .expect("Failed to execute ffmpeg");

    let movie_height = (movie_height as f64 * 540 as f64 / movie_width as f64).round() as u16;
    let movie_width: u16 = 540;

    if status.success() {
        println!("Processed: {} -> {}", input, output);
    } else {
        eprintln!("Failed to process: {}", input);
    }
    return (movie_width, movie_height);
}

fn main() {
    let args = Args::parse();
    let files: Vec<_> = fs::read_dir(args.source)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.eq_ignore_ascii_case("mp4"))
                .unwrap_or(false)
        })
        .collect();

    files.iter().for_each(|file| {
        let mut destination = args.destination.clone();
        destination.push('/');
        destination.push_str(file.file_name().to_str().unwrap());
        let (video_width, video_height) =
            process_video(file.path().to_str().unwrap(), destination.as_str());
        read_buffer(&destination, video_width, video_height);
    });
}
