use clap::Parser;

/// A downloader for telegram photo gallery
#[derive(Parser)]
struct Args {
    /// Directory to save the images
    #[arg(short, long)]
    dir: String,
    /// URLS of telegram graph gallery
    urls: Vec<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    for url in args.urls {
        telegram_photo_gallery_downloader::download(args.dir.clone(), url)
            .await
            .unwrap();
    }
}
