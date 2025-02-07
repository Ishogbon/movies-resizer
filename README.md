Below is an example `README.md` for the **movie-resizer** repository:

---

# Movie Resizer

Movie Resizer is a command-line tool written in Rust that resizes video files, extracts raw frame data, and generates metadata. It processes all `.mp4` files from a specified source directory, scales the videos to a width of 540 pixels (maintaining the aspect ratio), and then extracts the video frames as raw RGB data. Optionally, you can also output the frame data in JSON format.

## Features

- **Batch Processing:** Automatically process all `.mp4` files in a source directory.
- **Video Resizing:** Resizes videos to a fixed width of 540 pixels, preserving the aspect ratio.
- **Frame Extraction:** Extracts raw video frames with pixel format `rgb24`.
- **Metadata Generation:** Generates a metadata JSON file containing the video’s dimensions and scale.
- **Optional JSON Output:** Use the `--json` flag to generate JSON files that contain the extracted frame data.
- **Leverages ffmpeg/ffprobe:** Utilizes `ffmpeg` for video processing and `ffprobe` to retrieve video dimensions.

## Prerequisites

Before using Movie Resizer, make sure you have the following installed on your system:

- **Rust:** Install from [rust-lang.org](https://www.rust-lang.org/tools/install).
- **ffmpeg and ffprobe:** These are required for video processing. Installation instructions can be found on [ffmpeg.org](https://ffmpeg.org/download.html).

Additionally, the project uses the following Rust crates:
- [`clap`](https://crates.io/crates/clap) for command-line argument parsing.
- [`serde_json`](https://crates.io/crates/serde_json) for JSON serialization.

These dependencies will be automatically fetched when you build the project using Cargo.

## Installation

1. **Clone the repository:**

   ```bash
   git clone https://github.com/yourusername/movie-resizer.git
   cd movie-resizer
   ```

2. **Build the project:**

   Use Cargo to build the project in release mode:

   ```bash
   cargo build --release
   ```

3. **Locate the executable:**

   The compiled binary will be available at `./target/release/movie-resizer`.

## Usage

Movie Resizer processes videos from a specified source directory and outputs the processed videos along with associated metadata files in the destination directory.

### Command-Line Arguments

- `<source>`: The directory containing the `.mp4` files to process.
- `<destination>`: The directory where the processed videos and metadata files will be stored.
- `--json` or `-j`: *(Optional)* Flag to generate JSON files for the extracted frame data.

### Example Command

```bash
./target/release/movie-resizer ./input_videos ./output_videos --json
```

In this example:
- All `.mp4` files in `./input_videos` will be processed.
- Videos will be resized to a width of 540 pixels while maintaining their aspect ratio.
- Raw frame data (in RGB24 format) is extracted.
- If the `--json` flag is provided, a JSON file containing frame data is generated for each video.
- A metadata JSON file containing the video dimensions and scale is created alongside each processed video in `./output_videos`.

## How It Works

1. **Extract Video Dimensions:**  
   The tool uses `ffprobe` to query the video's width and height.

2. **Process Video with ffmpeg:**  
   Using `ffmpeg`, the video is resized to a width of 540 pixels, the frame rate is set to 1 fps, and processing is limited to the first 180 seconds. The output is generated as raw video data in the `rgb24` pixel format.

3. **Frame Buffering and JSON Generation:**  
   The raw video data is read into a buffer. If the JSON flag is enabled, the tool iterates through the frames, extracts RGB pixel values, and serializes them into a JSON file.

4. **Metadata Output:**  
   A separate metadata JSON file is created containing the resized video's width, height, and calculated scale (width × height × 3).

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! If you have suggestions, bug fixes, or new features, feel free to open an issue or submit a pull request.

## Contact

For any questions or issues, please contact [your email] or visit [your GitHub profile](https://github.com/yourusername).

---

Feel free to adjust any details (such as repository URLs, contact information, or additional usage instructions) to suit your project needs. Enjoy using Movie Resizer!
