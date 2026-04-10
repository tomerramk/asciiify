mod cli;
mod player;

use clap::Parser;
use asciiify_core::ConvertOptions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::Cli::parse();

    let opts = ConvertOptions {
        width: cli.width,
        height: cli.height,
        mode: cli::parse_mode(&cli.mode),
        invert: cli.invert,
        charset: cli.charset,
    };

    if cli::is_video(&cli.input) {
        #[cfg(feature = "video")]
        {
            player::play_video(&cli.input, &opts, cli.fps, cli.output.as_deref())?;
        }
        #[cfg(not(feature = "video"))]
        {
            eprintln!("Video support not compiled in. Rebuild with --features video");
            std::process::exit(1);
        }
    } else {
        let art = asciiify_core::convert_image_file(&cli.input, &opts)?;
        if let Some(ref out_path) = cli.output {
            std::fs::write(out_path, &art)?;
            eprintln!("Written to {out_path}");
        } else {
            println!("{art}");
        }
    }

    Ok(())
}
