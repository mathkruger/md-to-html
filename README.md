# md-to-html

This project converts markdown text into HTML. I use it to create quick docs at work since I'm too lazy to write a real doc and I prefer writing it in markdown.

You need to install [wkhtmltopdf](https://wkhtmltopdf.org/downloads.html) before building/installing.

## Instalation (Linux)
For now, there's only a linux installation script. If you are on mac, I think it will work. If you are on windows, create your own installation script or build from source and do whatever you want with it (Or use WSL. Even better, move to Linux for God's sake)

``` bash
sh install.sh
```

## Usage

``` bash
md-to-html /path/to/file.md /path/to/output.html|pdf dark|light pdf|html
```

## Build

``` bash
# On dev mode
cargo build

# On release mode
cargo build --release
```
