use pulldown_cmark::{Parser, Event, Tag, TagEnd, Options, CodeBlockKind, HeadingLevel};
use std::path::PathBuf;

const CSS: &str = r#"@media print {
@page {
size: a4;
margin: 2.5cm !important;
}
body {
font-family: "Georgia", "Times New Roman", serif;
font-size: 11pt;
line-height: 1.6;
color: #1a1a1a;
max-width: 100%;
}
table {
width: 100% !important;
border-collapse: collapse !important;
margin: 1.5rem 0 !important;
page-break-inside: avoid !important;
break-inside: avoid-page !important;
}
th, td {
border: 1px solid #dedede !important;
padding: 10px !important;
text-align: left !important;
}
th {
background-color: #f7f7f7 !important;
font-weight: bold;
}
blockquote {
display: block !important;
page-break-inside: avoid !important;
break-inside: avoid-page !important;
border-left: 3px solid #d0d0d0;
padding-left: 12pt;
margin: 1.5rem 0;
color: #555;
}
pre {
display: block !important;
page-break-inside: avoid !important;
break-inside: avoid-page !important;
background: #f5f5f5;
padding: 1rem;
border-radius: 4px;
overflow-x: auto;
margin: 1.5rem 0;
}
code {
font-family: "Courier New", monospace;
font-size: 0.9em;
}
pre code {
background: none;
padding: 0;
border-radius: 0;
}
img {
max-width: 100%;
height: auto;
}
h1, h2, h3, h4, h5, h6 {
color: #111111;
page-break-after: avoid;
}
ul, ol {
margin: 1rem 0;
padding-left: 2rem;
}
li {
margin: 0.4rem 0;
}
p {
margin: 0.8rem 0;
}
}"#;

pub fn render(markdown: &str) -> Result<Vec<u8>, String> {
    let html_body = md_to_html(markdown);
    let full_html = format!(
        "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n<style>{}</style>\n</head>\n<body>\n{}</body>\n</html>",
        CSS, html_body
    );
    html_to_pdf(&full_html)
}

fn escape_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            c => out.push(c),
        }
    }
    out
}

fn heading_num(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

fn md_to_html(md: &str) -> String {
    let mut out = String::new();
    let mut ordered_list = false;
    let mut in_code_block = false;
    let mut in_table_head = false;
    let mut table_body_open = false;
    let mut in_image = false;
    let mut image_alt = String::new();

    let parser = Parser::new_ext(md, Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TASKLISTS | Options::ENABLE_FOOTNOTES | Options::ENABLE_HEADING_ATTRIBUTES | Options::ENABLE_MATH);

    for event in parser {
        match event {
            Event::Start(tag) => match tag {
                Tag::Heading { level, .. } => {
                    let n = heading_num(level);
                    out.push_str("<h");
                    out.push((b'0' + n) as char);
                    out.push('>');
                }
                Tag::Paragraph => {
                    out.push_str("<p>");
                }
                Tag::BlockQuote(_kind) => {
                    out.push_str("<blockquote>\n");
                }
                Tag::CodeBlock(kind) => {
                    in_code_block = true;
                    let lang = match kind {
                        CodeBlockKind::Fenced(info) => info.trim().to_string(),
                        CodeBlockKind::Indented => String::new(),
                    };
                    out.push_str("<pre><code");
                    if !lang.is_empty() {
                        out.push_str(" class=\"language-");
                        out.push_str(&lang);
                        out.push('"');
                    }
                    out.push('>');
                }
                Tag::List(start) => {
                    ordered_list = start.is_some();
                    if ordered_list {
                        out.push_str("<ol>\n");
                    } else {
                        out.push_str("<ul>\n");
                    }
                }
                Tag::Item => {
                    out.push_str("<li>");
                }
                Tag::Emphasis => out.push_str("<em>"),
                Tag::Strong => out.push_str("<strong>"),
                Tag::Strikethrough => out.push_str("<del>"),
                Tag::Link { dest_url, .. } => {
                    out.push_str("<a href=\"");
                    out.push_str(&dest_url);
                    out.push_str("\">");
                }
                Tag::Image { dest_url, .. } => {
                    in_image = true;
                    image_alt.clear();
                    out.push_str("<img src=\"");
                    out.push_str(&dest_url);
                    out.push_str("\" alt=\"");
                }
                Tag::Table(_alignments) => {
                    out.push_str("<table>\n");
                    out.push_str("<thead>\n");
                    in_table_head = true;
                }
                Tag::TableHead => {}
                Tag::TableRow => {
                    out.push_str("<tr>");
                }
                Tag::TableCell => {
                    if in_table_head {
                        out.push_str("<th>");
                    } else {
                        out.push_str("<td>");
                    }
                }
                Tag::FootnoteDefinition(_) => {}
                _ => {}
            },
            Event::End(tag) => match tag {
                TagEnd::Heading(level) => {
                    let n = heading_num(level);
                    out.push_str("</h");
                    out.push((b'0' + n) as char);
                    out.push_str(">\n");
                }
                TagEnd::Paragraph => {
                    out.push_str("</p>\n\n");
                }
                TagEnd::BlockQuote(_kind) => {
                    out.push_str("</blockquote>\n\n");
                }
                TagEnd::CodeBlock => {
                    in_code_block = false;
                    out.push_str("</code></pre>\n\n");
                }
                TagEnd::List(_ordered) => {
                    if ordered_list {
                        out.push_str("</ol>\n");
                    } else {
                        out.push_str("</ul>\n");
                    }
                    ordered_list = false;
                }
                TagEnd::Item => {
                    out.push_str("</li>\n");
                }
                TagEnd::Emphasis => out.push_str("</em>"),
                TagEnd::Strong => out.push_str("</strong>"),
                TagEnd::Strikethrough => out.push_str("</del>"),
                TagEnd::Link => out.push_str("</a>"),
                TagEnd::Image => {
                    in_image = false;
                    out.push_str(&image_alt);
                    out.push_str("\">");
                }
                TagEnd::Table => {
                    if table_body_open {
                        out.push_str("</tbody>\n");
                    }
                    out.push_str("</table>\n\n");
                    in_table_head = false;
                    table_body_open = false;
                }
                TagEnd::TableHead => {
                    in_table_head = false;
                    out.push_str("</tr>\n</thead>\n<tbody>\n");
                    table_body_open = true;
                }
                TagEnd::TableRow => {
                    out.push_str("</tr>\n");
                }
                TagEnd::TableCell => {
                    if in_table_head {
                        out.push_str("</th>");
                    } else {
                        out.push_str("</td>");
                    }
                }
                _ => {}
            },
            Event::Text(text) => {
                if in_image {
                    image_alt.push_str(&text);
                } else if in_code_block {
                    out.push_str(&escape_html(&text));
                } else {
                    out.push_str(&escape_html(&text));
                }
            }
            Event::Code(text) => {
                out.push_str("<code>");
                out.push_str(&escape_html(&text));
                out.push_str("</code>");
            }
            Event::SoftBreak => {
                if in_code_block {
                    out.push('\n');
                } else {
                    out.push('\n');
                }
            }
            Event::HardBreak => {
                out.push_str("<br>\n");
            }
            Event::Rule => {
                out.push_str("<hr>\n\n");
            }
            Event::Html(html) | Event::InlineHtml(html) => {
                out.push_str(&escape_html(&html));
            }
            Event::FootnoteReference(_) => {}
            Event::InlineMath(_) | Event::DisplayMath(_) | Event::TaskListMarker(_) => {}
        }
    }

    out
}

fn html_to_pdf(html: &str) -> Result<Vec<u8>, String> {
    use headless_chrome::{Browser, LaunchOptions};
    use headless_chrome::types::PrintToPdfOptions;

    let tmp_dir = std::env::temp_dir().join(format!("md_pdf_{}", std::process::id()));
    std::fs::create_dir_all(&tmp_dir).map_err(|e| format!("Temp dir: {}", e))?;
    let html_path = tmp_dir.join("document.html");
    std::fs::write(&html_path, html).map_err(|e| format!("Write temp: {}", e))?;
    let file_url = format!("file:///{}", html_path.to_string_lossy().replace('\\', "/"));

    let launch = |path: Option<PathBuf>| -> Result<Vec<u8>, String> {
        let opts = LaunchOptions {
            headless: true,
            sandbox: false,
            path,
            ..Default::default()
        };
        let browser = Browser::new(opts).map_err(|e| e.to_string())?;
        let tab = browser.new_tab().map_err(|e| e.to_string())?;
        tab.navigate_to(&file_url).map_err(|e| e.to_string())?;
        tab.wait_until_navigated().map_err(|e| e.to_string())?;
        let pdf_opts = PrintToPdfOptions {
            print_background: Some(true),
            prefer_css_page_size: Some(true),
            ..Default::default()
        };
        tab.print_to_pdf(Some(pdf_opts)).map_err(|e| e.to_string())
    };

    let result = launch(None).or_else(|_| {
        let edge = find_edge();
        match edge {
            Some(path) => launch(Some(path)),
            None => Err("No browser found".to_string()),
        }
    });

    let _ = std::fs::remove_dir_all(&tmp_dir);
    result.map_err(|e| format!("Headless browser PDF failed: {}. Ensure Google Chrome or Microsoft Edge is installed.\nDetail: {}", e, e))
}

fn find_edge() -> Option<PathBuf> {
    for p in [
        r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
        r"C:\Program Files\Microsoft\Edge\Application\msedge.exe",
    ] {
        let path = PathBuf::from(p);
        if path.exists() {
            return Some(path);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text() {
        let html = md_to_html("Hello world");
        assert!(html.contains("<p>Hello world</p>"));
    }

    #[test]
    fn heading() {
        let html = md_to_html("# H1\n## H2\n### H3");
        assert!(html.contains("<h1>H1</h1>"));
        assert!(html.contains("<h2>H2</h2>"));
        assert!(html.contains("<h3>H3</h3>"));
    }

    #[test]
    fn emphasis_and_strong() {
        let html = md_to_html("*italic* **bold**");
        assert!(html.contains("<em>italic</em>"));
        assert!(html.contains("<strong>bold</strong>"));
    }

    #[test]
    fn strikethrough() {
        let html = md_to_html("~~struck~~");
        assert!(html.contains("<del>struck</del>"));
    }

    #[test]
    fn html_escaping() {
        let html = md_to_html("<angle> & \"quote\"");
        assert!(html.contains("&lt;angle&gt;"));
        assert!(html.contains(" &amp; "));
        assert!(html.contains("\"quote\""));
    }

    #[test]
    fn links() {
        let html = md_to_html("[click](https://x.com)");
        assert!(html.contains("<a href=\"https://x.com\">click</a>"));
    }

    #[test]
    fn unordered_list() {
        let html = md_to_html("- a\n- b");
        assert!(html.contains("<ul>"));
        assert!(html.contains("<li>a</li>"));
        assert!(html.contains("<li>b</li>"));
        assert!(html.contains("</ul>"));
    }

    #[test]
    fn ordered_list() {
        let html = md_to_html("1. one\n2. two");
        assert!(html.contains("<ol>"));
        assert!(html.contains("<li>one</li>"));
        assert!(html.contains("</ol>"));
    }

    #[test]
    fn code_block() {
        let html = md_to_html("```rust\nfn main() {}\n```");
        assert!(html.contains("<pre><code class=\"language-rust\">"));
        assert!(html.contains("fn main() {}"));
        assert!(html.contains("</code></pre>"));
    }

    #[test]
    fn inline_code() {
        let html = md_to_html("use `code` here");
        assert!(html.contains("<code>code</code>"));
    }

    #[test]
    fn blockquote() {
        let html = md_to_html("> quoted");
        assert!(html.contains("<blockquote>"));
        assert!(html.contains("quoted"));
        assert!(html.contains("</blockquote>"));
    }

    #[test]
    fn table() {
        let html = md_to_html("| H1 | H2 |\n|---|----|\n| A | B |");
        assert!(html.contains("<table>"));
        assert!(html.contains("<thead>"));
        assert!(html.contains("<th>H1</th>"));
        assert!(html.contains("<tbody>"));
        assert!(html.contains("<td>A</td>"));
        assert!(html.contains("</table>"));
    }

    #[test]
    fn horizontal_rule() {
        let html = md_to_html("---");
        assert!(html.contains("<hr>"));
    }

    #[test]
    fn image_alt() {
        let html = md_to_html("![alt](img.png)");
        assert!(html.contains("<img src=\"img.png\" alt=\"alt\">"));
    }

    #[test]
    fn hard_break() {
        let html = md_to_html("line1  \nline2");
        assert!(html.contains("<br>"));
    }
}
