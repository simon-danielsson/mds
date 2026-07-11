use crate::{parse::Section, statics::templates::*};

pub fn generate(slideshow: Vec<Section>) -> anyhow::Result<String> {
    let mut file = String::new();

    file.push_str(HTML_HEADER);

    // css
    {
        file.push_str("<style>");
        file.push_str(CSS);
        file.push_str("</style>");
    }

    // js
    {
        file.push_str("<script>");
        file.push_str("insert js and style here");
        file.push_str("</script>");
    }

    // title
    {
        file.push_str("<title>");
        if let Some(name) = &slideshow[0].name {
            file.push_str(&name);
        }
        file.push_str("</title>\n</head>");
    }

    // body
    {
        file.push_str("<body>");
        file.push_str("insert body here");
        file.push_str("</body>\n</html>");
    }

    Ok(file)
}
