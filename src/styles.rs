use md_to_html::{read_file, get_current_path};

pub struct Style {
}

impl Style {
    pub fn get(style_name: &str) -> String {
        let css_path = get_current_path() + "/css";
        match style_name {
            "dark" => {
                read_file(&format!("{css_path}/dark.css"))
            },
            "light" => {
                read_file(&format!("{css_path}/light.css"))
            },
            _ => {
                read_file(&format!("{css_path}/dark.css"))
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
