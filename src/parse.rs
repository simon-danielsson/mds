// Slide hierarchy:
//
//     # h1            slide section
//     ## h2           slide
//     ### h3          slide header text
//     #### h4         slide sub-header

#[derive(Clone, Debug)]
pub enum TextItem {
    Header(String),
    SubHeader(String),
    Body(String),
    Quote(String),
    Code(String),
}

#[derive(Clone, Debug)]
pub enum ListItem {
    Bullet(String),
    Number(String),
    Check((String, bool)),
}

#[derive(Clone, Debug)]
pub enum SlideItem {
    Text(TextItem),
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

fn parse_section(
    slideshow: &Vec<Section>,
    mut md_lines: &mut std::str::Lines<'_>,
) -> anyhow::Result<Vec<Section>> {
    let mut slideshow = slideshow.clone();
    while let Some(line) = md_lines.next() {
        match line {
            // new section
            _ if line.trim_start().starts_with("# ") => {
                println!("new slide section: {line}");
                slideshow.push(Section::new());
                if !line.split_at(2).1.is_empty() {
                    if let Some(a) = slideshow.last_mut() {
                        a.name = Some(line.split_at(2).1.to_string());
                        // println!("{:#?}", a.name); // debug
                    }
                    // println!("{:#?}", slideshow.last_mut().unwrap().name); // debug
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
                println!("new slide: {line}");
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
                println!("slide header: {line}");
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
                println!("slide sub-header: {line}");
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
                println!("quote: {line}");
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

                // println!("bullet-list item: {line}");
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
                // unimplemented!("check list: {line}");
                // println!("check-list item: {line}");
            }

            // numbered list item
            _ if !line.is_empty() => {
                if line.trim().as_bytes()[0].is_ascii_digit() {
                    if line.trim().as_bytes()[1] == b'.'
                        && line.trim().as_bytes()[2] == b' '
                    {
                        if let Some(a) = slideshow.last_mut() {
                            if let Some(a) = a.slides.last_mut() {
                                a.push_list_item(ListItem::Number(
                                        line.split_at(3)
                                        .1
                                        .trim()
                                        .to_string(),
                                ));
                            }
                        }
                        // unimplemented!("numbered list: {line}");
                    }
                    // println!("check-list item: {line}");
                }
            }

            _ => {
                if let Some(a) = slideshow.last_mut() {
                    if let Some(a) = a.slides.last_mut() {
                        let si = SlideItem::Text(TextItem::Body(
                                line.split_at(0).1.trim().to_string(),
                        ));
                        a.items.push(si);
                    }
                }
                println!("body: {line}");
            }
        }
    }
    Ok(slideshow.clone())
}
