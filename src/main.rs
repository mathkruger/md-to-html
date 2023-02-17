mod styles;

use comrak::{markdown_to_html, ComrakOptions, ComrakExtensionOptions};
use md_to_html::{read_file, write_file, Args, save_html_as_pdf};
use styles::{add_html_styling, Style};

fn main() -> () {
    let args = Args::get();
    
    let text = read_file(&args.input);
    let styling = Style::get(&args.style);
    let options = ComrakOptions {
        extension: ComrakExtensionOptions {
            table: true,
            tasklist: true,
            ..ComrakExtensionOptions::default()
        },
        ..ComrakOptions::default()
    };

    let content = markdown_to_html(&text, &options);
    
    let markdown = add_html_styling(
        &content,
        &styling
    );

    if args.format == "pdf" {
        save_html_as_pdf(&content, &args.input, &args.output)
    } else {
        write_file(&args.output, markdown)
    }
}
