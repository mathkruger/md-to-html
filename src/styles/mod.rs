use md_to_html::read_file;

pub struct Style {
}

impl Style {
    pub fn get(style_name: &str) -> String {
        match style_name {
            "dark" => {
                read_file("css/dark.css")
            },
            "light" => {
                read_file("css/light.css")
            },
            _ => {
                read_file("css/dark.css")
            }
        }
    }
}

pub fn add_html_styling(html: &String, style: &String) -> String {
    format!(r##"
        <html>
            <head>
                <style>{style}</style>
            </head>
            <body class="markdown-body">
                {html}
            </body>
        </html>
    "##)
}
