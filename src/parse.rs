// Slide hierarchy:
//
//     # h1            slide section
//     ## h2           slide
//     ### h3          slide header text
//     #### h4         slide sub-header

use std::{path::PathBuf, str::FromStr};

use anyhow::anyhow;

#[derive(Clone, Debug)]
pub enum TextItem {
    Header(String),
    SubHeader(String),
    Body(String),
    Quote(String),
    /// String name, String content
    Code((String, String)),
    /// String link name, String link url
    Link((String, String)),
}

#[derive(Clone, Debug)]
pub enum ListItem {
    Bullet(String),
    Number(String),
    /// String content, bool checkmark
    Check((String, bool)),
}

#[derive(Clone, Debug)]
pub enum SlideItem {
    Text(TextItem),
    /// String name, PathBuf filepath
    Image((String, PathBuf)),
    List(Vec<ListItem>),
}

#[derive(Clone, Debug, Default)]
pub struct Slide {
    name: Option<String>,
    items: Vec<SlideItem>,
}

impl Slide {
    fn new(name: Option<String>) -> Self {
        Self {
            name,
            items: Vec::new(),
        }
    }
    fn push_list_item(&mut self, item: ListItem) {
        match self.items.last_mut() {
            Some(SlideItem::List(list)) => {
                list.push(item);
            }
            _ => {
                self.items.push(SlideItem::List(vec![item]));
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Section {
    name: Option<String>,
    slides: Vec<Slide>,
}
impl Section {
    fn new() -> Self {
        Self {
            name: None,
            slides: Vec::new(),
        }
    }
}

pub fn md_parse(md_contents: String) -> anyhow::Result<Vec<Section>> {
    let mut slideshow: Vec<Section> = Vec::new();
    // println!("{}", md_contents);

    let mut md_lines = md_contents.lines().into_iter();
    slideshow = parse_section(&slideshow, &mut md_lines)?;

    Ok(slideshow)
}

/// recursive
fn parse_section(
    slideshow: &Vec<Section>,
    mut md_lines: &mut std::str::Lines<'_>,
) -> anyhow::Result<Vec<Section>> {
    let mut slideshow = slideshow.clone();
    while let Some(line) = md_lines.next() {
        match line {
            // new section
            _ if line.trim_start().starts_with("# ") => {
                slideshow.push(Section::new());
                if !line.split_at(2).1.is_empty() {
                    if let Some(a) = slideshow.last_mut() {
                        a.name = Some(line.split_at(2).1.to_string());
                    }
                }
                slideshow = parse_section(&slideshow, &mut md_lines)?;
            }

            // new slide
            _ if line.trim_start().starts_with("## ") => {
                if let Some(a) = slideshow.last_mut() {
                    let mut name: Option<String> =
                        Some(line.split_at(2).1.trim().to_string());
                    if name.clone().unwrap().is_empty() {
                        name = None;
                    }
                    a.slides.push(Slide::new(name));
                }
            }

            // text header
            _ if line.trim_start().starts_with("### ") => {
                if let Some(a) = slideshow.last_mut() {
                    if let Some(a) = a.slides.last_mut() {
                        let si = SlideItem::Text(TextItem::Header(
                                line.split_at(3).1.trim().to_string(),
                        ));
                        a.items.push(si);
                    }
                }
            }

            // text sub-header
            _ if line.trim_start().starts_with("#### ") => {
                if let Some(a) = slideshow.last_mut() {
                    if let Some(a) = a.slides.last_mut() {
                        let si = SlideItem::Text(TextItem::SubHeader(
                                line.split_at(4).1.trim().to_string(),
                        ));
                        a.items.push(si);
                    }
                }
            }

            // quote
            _ if line.trim_start().starts_with("> ") => {
                if let Some(a) = slideshow.last_mut() {
                    if let Some(a) = a.slides.last_mut() {
                        let si = SlideItem::Text(TextItem::Quote(
                                line.split_at(1).1.trim().to_string(),
                        ));
                        a.items.push(si);
                    }
                }
            }

            // code
            _ if line.trim_start().starts_with("```") => {
                let lang_name = line
                    .get(4..)
                    .map(|s| s.to_string())
                    .unwrap_or("terminal".to_string());
                println!("lang: {}", lang_name);
                let mut content = String::new();
                'get_content: while let Some(line) = md_lines.next() {
                    if line.trim_start().starts_with("```") {
                        break 'get_content;
                    }
                    content.push_str(line);
                }
                if let Some(a) = slideshow.last_mut() {
                    if let Some(a) = a.slides.last_mut() {
                        let si = SlideItem::Text(TextItem::Code((
                                    lang_name, content,
                        )));
                        a.items.push(si);
                    }
                }
            }

            // image
            _ if line.trim_start().starts_with("![") => {
                let mut line_c_it = line.chars();
                let mut name = String::new();
                let mut path = String::new();
                while let Some(c) = line_c_it.next() {
                    if c == '\n' {
                        return Err(anyhow!(
                                "premature EOL whilst parsing image item"
                        ));
                    }
                    if c == '[' {
                        while let Some(ci) = line_c_it.next() {
                            if ci == ']' {
                                break;
                            }
                            name.push(ci);
                        }
                    }
                    if c == '(' {
                        while let Some(ci) = line_c_it.next() {
                            if ci == ')' || c == '\n' {
                                break;
                            }
                            path.push(ci);
                        }
                    }
                }

                path = resolve_path_str(path)?;
                let path_b = PathBuf::from_str(&path)?.canonicalize()?;

                if !path_b.is_file() {
                    return Err(anyhow!(
                            "image item \"{}\" could not be found or is not a valid file",
                            path_b.display()
                    ));
                }

                if let Some(a) = slideshow.last_mut() {
                    if let Some(a) = a.slides.last_mut() {
                        let si = SlideItem::Image((name, path_b));
                        a.items.push(si);
                    }
                }
            }

            // bullet list item
            _ if (line.trim_start().starts_with("- ")
                && !line.trim_start().starts_with("- ["))
                || line.trim_start().starts_with("+ ")
                || line.trim_start().starts_with("* ") =>
            {
                if let Some(a) = slideshow.last_mut() {
                    if let Some(a) = a.slides.last_mut() {
                        a.push_list_item(ListItem::Bullet(
                                line.split_at(2).1.trim().to_string(),
                        ));
                    }
                }
            }

            // check list item
            _ if line.trim_start().starts_with("- [") => {
                let mut checked = false;
                if line.trim().as_bytes()[3] == b'x' {
                    checked = true;
                }

                if let Some(a) = slideshow.last_mut() {
                    if let Some(a) = a.slides.last_mut() {
                        a.push_list_item(ListItem::Check((
                                    line.split_at(5).1.trim().to_string(),
                                    checked,
                        )));
                    }
                }
            }

            // link
            _ if (line.trim_start().contains('[')
                && line.trim_start().contains(']'))
                && line.trim_start().contains('(')
                && line.trim_start().contains(')') =>
            {
                let mut line_c_it = line.chars();
                let mut name = String::new();
                let mut url = String::new();

                while let Some(c) = line_c_it.next() {
                    if c == '\n' {
                        return Err(anyhow!(
                                "premature EOL whilst parsing link item"
                        ));
                    }

                    if c == '[' {
                        while let Some(ci) = line_c_it.next() {
                            if ci == ']' {
                                break;
                            }
                            name.push(ci);
                        }
                    }
                    if c == '(' {
                        while let Some(ci) = line_c_it.next() {
                            if ci == ')' || c == '\n' {
                                break;
                            }
                            url.push(ci);
                        }
                    }
                }

                if let Some(a) = slideshow.last_mut() {
                    if let Some(a) = a.slides.last_mut() {
                        let si = SlideItem::Text(TextItem::Link((
                                    name, url,
                        )));
                        a.items.push(si);
                    }
                }
            }

            // numbered list item
            _ if !line.is_empty()
                && line.trim().as_bytes()[0].is_ascii_digit()
                && line.trim().as_bytes()[1] == b'.'
                && line.trim().as_bytes()[2] == b' ' =>
            {
                if let Some(a) = slideshow.last_mut() {
                    if let Some(a) = a.slides.last_mut() {
                        a.push_list_item(ListItem::Number(
                                line.split_at(3).1.trim().to_string(),
                        ));
                    }
                }
            }

            // body
            _ => {
                if let Some(a) = slideshow.last_mut() {
                    if let Some(a) = a.slides.last_mut() {
                        let si = SlideItem::Text(TextItem::Body(
                                line.split_at(0).1.trim().to_string(),
                        ));
                        a.items.push(si);
                    }
                }
            }
        }
    }
    Ok(slideshow.clone())
}

fn resolve_path_str(mut path: String) -> anyhow::Result<String> {
    if path.starts_with("~") {
        let home = std::env::var("HOME")?;
        path = path.replace("~", &home);
    }

    if path.starts_with(".") {
        path = path.replace(
            "~",
            std::env::current_dir()?
            .to_str()
            .expect("could not expand relative path of \"{path}\""),
        );
    }
    Ok(path)
}
