use std::env;
use std::process::Command;
use versalogrs::NewVersaLog;

fn main() {
    let logger = NewVersaLog(
        "detailed",               // mode
        false,                    // show_file
        true,                     // show_tag
        "Twitter Video Download", // tag
        false,                    // enable_all
        false,                    // notice
        false,                    // warning
        vec![],                   // prefix (Vec<String>)
        false,                    // error
    );

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        logger.error("No URL provided", &[]);
        return;
    }

    let mut url = args[1].clone();

    // ✅ x.com / twitter.com / 埋め込みURL対応
    if url.contains("x.com") || url.contains("twitter.com") {
        url = url
            .replace("mobile.twitter.com", "x.com")
            .replace("twitter.com", "x.com")
            .replace("https://www.x.com", "https://x.com")
            .replace("https://fxtwitter.com", "https://x.com")
            .replace("https://fixupx.com", "https://x.com");
    } else {
        logger.error("Invalid URL", &[]);
        return;
    }

    logger.info(&format!("Downloading from: {}", url), &[]);

    let status = Command::new("python")
        .args([
            "-m",
            "yt_dlp",
            "-f",
            "best[ext=mp4]",
            "-o",
            "%(title)s.%(ext)s",
            &url,
        ])
        .status();

    match status {
        Ok(s) if s.success() => {
            logger.info("Download complete", &[]);
        }
        Ok(s) => {
            logger.error(&format!("yt-dlp exited with status: {:?}", s), &[]);
        }
        Err(e) => {
            logger.error(&format!("Failed to execute yt-dlp: {:?}", e), &[]);
        }
    }
}
