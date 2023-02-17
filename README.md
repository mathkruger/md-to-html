# md-to-html

This projects converts a markdown text into html. I use it to create quick docs at work, since I'm too lazy to write a real doc and I prefer writing it in markdown.

## Usage

``` bash
md-to-html /path/to/file.md /path/to/output.html|pdf dark|light pdf|html
```

## Build

You need to install [wkhtmltopdf](https://wkhtmltopdf.org/downloads.html) before building this source code.

``` bash
# On dev mode
cargo build

# On release mode
cargo build --release
```
