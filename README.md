On telegram, there are many groups share pictures or photos through gallery in links like https://telegra.ph/xxx, and usually you want to download them for they are so beautiful and useful.

# Crate
This can be used as a crate;

features:
- indicatif: to show the process bar, turned on by default

# CLI
Run `cargo build --release` to build the CLI.

```sh
$ telegram_photo_gallery_downloader -h
A downloader for telegram photo gallery

Usage: telegram_photo_gallery_downloader --dir <DIR> [URLS]...

Arguments:
  [URLS]...  URLS of telegram graph gallery

Options:
  -d, --dir <DIR>  Directory to save the images
  -h, --help       Print help
```
