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
        file.push_str(JS);
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
        file.push_str("<div class=\"progress-bar\"></div>");
        file.push_str("<div class=\"content\">");
        file.push_str(&intermediary_to_html(slideshow)?);
        file.push_str("</div>\n</body>\n</html>");
    }

    Ok(file)
}

fn intermediary_to_html(slideshow: Vec<Section>) -> anyhow::Result<String> {
    let mut op = String::new();
    for section in slideshow {
        // section div
        if let Some(name) = section.name {
            op.push_str(
                format!("\n<div class=\"section\" data-title=\"{}\">", name)
                .as_str(),
            );
        } else {
            op.push_str("\n<div class=\"section\" data-title=\"\">");
        }

        // slide divs
        for slide in section.slides {
            if let Some(name) = slide.name {
                op.push_str(
                    format!("\n<div class=\"slide\" data-title=\"{}\">", name)
                    .as_str(),
                );
            } else {
                op.push_str("<div class=\"section\" data-title=\"\">");
            }

            for item in slide.items {
                op.push_str(&item.to_html())
            }

            // div class="slide"
            op.push_str("\n</div>");
        }
        // div class="section"
        op.push_str("\n</div>");
    }
    Ok(op)
}
