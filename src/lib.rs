use futures::future::{BoxFuture, FutureExt};
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::IntoUrl;
use std::fmt::{Debug, Write};
use ttf_parser::{Face, GlyphId, OutlineBuilder};

lazy_static! {
    static ref FONT_FACE_REGEX: Regex = Regex::new(r"@font-face\s*\{([^}]+)\}").unwrap();
    static ref FONT_FACE_URL_REGEX: Regex = Regex::new(r"url\(([^)]+)\)").unwrap();
    static ref CSS_IMPORT_REGEX: Regex =
        Regex::new(r#"(?i)@import\s+url\s*\(\s*(?:"([^"]+)"|'([^']+)'|\(([^)]+)\))\s*\)\s*;"#)
            .unwrap();
}

type AnyError = Box<dyn std::error::Error>;
type AnyResult<T> = Result<T, AnyError>;

struct Builder<'a>(&'a mut String);

impl OutlineBuilder for Builder<'_> {
    fn move_to(&mut self, x: f32, y: f32) {
        write!(self.0, "M {} {} ", x, y).unwrap()
    }

    fn line_to(&mut self, x: f32, y: f32) {
        write!(self.0, "L {} {} ", x, y).unwrap()
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        write!(self.0, "Q {} {} {} {} ", x1, y1, x, y).unwrap()
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        write!(self.0, "C {} {} {} {} {} {} ", x1, y1, x2, y2, x, y).unwrap()
    }

    fn close(&mut self) {
        self.0.push_str("Z ")
    }
}

struct Vector {
    x: f32,
    y: f32,
}

impl Debug for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec({}, {})", self.x, self.y)
    }
}

impl Default for Vector {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl Vector {
    fn into_absolute_rect(self) -> Rect {
        Rect {
            start: Vector::default(),
            end: self,
        }
    }
}

#[derive(Debug)]
struct Rect {
    start: Vector,
    end: Vector,
}

#[derive(Debug)]
struct RemappedBuilder<'a> {
    buf: &'a mut String,
    input_rect: Rect,
    output_rect: Rect,
}

impl OutlineBuilder for RemappedBuilder<'_> {
    fn move_to(&mut self, x: f32, y: f32) {
        write!(
            self.buf,
            "M {} {} ",
            self.output_rect.start.x
                + ((x - self.input_rect.start.x)
                    * (self.output_rect.end.x - self.output_rect.start.x)
                    / (self.input_rect.end.x - self.input_rect.start.x)),
            self.output_rect.start.y
                + ((self.input_rect.end.y - y)
                    * (self.output_rect.end.y - self.output_rect.start.y)
                    / (self.input_rect.end.y - self.input_rect.start.y))
        )
        .unwrap()
    }

    fn line_to(&mut self, x: f32, y: f32) {
        write!(
            self.buf,
            "L {} {} ",
            self.output_rect.start.x
                + ((x - self.input_rect.start.x)
                    * (self.output_rect.end.x - self.output_rect.start.x)
                    / (self.input_rect.end.x - self.input_rect.start.x)),
            self.output_rect.start.y
                + ((self.input_rect.end.y - y)
                    * (self.output_rect.end.y - self.output_rect.start.y)
                    / (self.input_rect.end.y - self.input_rect.start.y))
        )
        .unwrap()
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        write!(
            self.buf,
            "Q {} {} {} {} ",
            self.output_rect.start.x
                + ((x1 - self.input_rect.start.x)
                    * (self.output_rect.end.x - self.output_rect.start.x)
                    / (self.input_rect.end.x - self.input_rect.start.x)),
            self.output_rect.start.y
                + ((self.input_rect.end.y - y1)
                    * (self.output_rect.end.y - self.output_rect.start.y)
                    / (self.input_rect.end.y - self.input_rect.start.y)),
            self.output_rect.start.x
                + ((x - self.input_rect.start.x)
                    * (self.output_rect.end.x - self.output_rect.start.x)
                    / (self.input_rect.end.x - self.input_rect.start.x)),
            self.output_rect.start.y
                + ((self.input_rect.end.y - y)
                    * (self.output_rect.end.y - self.output_rect.start.y)
                    / (self.input_rect.end.y - self.input_rect.start.y))
        )
        .unwrap()
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        write!(
            self.buf,
            "C {} {} {} {} {} {} ",
            self.output_rect.start.x
                + ((x1 - self.input_rect.start.x)
                    * (self.output_rect.end.x - self.output_rect.start.x)
                    / (self.input_rect.end.x - self.input_rect.start.x)),
            self.output_rect.start.y
                + ((self.input_rect.end.y - y1)
                    * (self.output_rect.end.y - self.output_rect.start.y)
                    / (self.input_rect.end.y - self.input_rect.start.y)),
            self.output_rect.start.x
                + ((x2 - self.input_rect.start.x)
                    * (self.output_rect.end.x - self.output_rect.start.x)
                    / (self.input_rect.end.x - self.input_rect.start.x)),
            self.output_rect.start.y
                + ((self.input_rect.end.y - y2)
                    * (self.output_rect.end.y - self.output_rect.start.y)
                    / (self.input_rect.end.y - self.input_rect.start.y)),
            self.output_rect.start.x
                + ((x - self.input_rect.start.x)
                    * (self.output_rect.end.x - self.output_rect.start.x)
                    / (self.input_rect.end.x - self.input_rect.start.x)),
            self.output_rect.start.y
                + ((self.input_rect.end.y - y)
                    * (self.output_rect.end.y - self.output_rect.start.y)
                    / (self.input_rect.end.y - self.input_rect.start.y))
        )
        .unwrap()
    }

    fn close(&mut self) {
        self.buf.push_str("Z ")
    }
}

fn ext(file_name: &str) -> Option<String> {
    file_name
        .split('?')
        .next()
        .unwrap()
        .split('#')
        .next()
        .unwrap()
        .split('.')
        .last()
        .map(|ext| ext.to_string())
}

fn normalize(string: &str) -> String {
    let tmp: Vec<_> = string
        .split('.')
        .find(|part| !part.is_empty())
        .unwrap()
        .split('/')
        .filter(|part| !part.is_empty())
        .collect();

    let tmp = tmp.last().unwrap_or(tmp.last().unwrap());

    let tmp = if let Some(stripped) = tmp.strip_prefix("fa-") {
        stripped.to_string()
    } else {
        tmp.to_string()
    };

    let tmp: Vec<_> = tmp
        .split('?')
        .next()
        .unwrap()
        .split('#')
        .next()
        .unwrap()
        .split('-')
        .filter(|part| part.parse::<u32>().is_err())
        .collect();

    tmp.join("-")
}

fn font_to_svg(face: &mut Face) -> Vec<(String, String)> {
    let mut path_buf = String::with_capacity(256);
    let mut svgs = Vec::new();
    let output_dim = 512.0;

    for id in 0..face.number_of_glyphs() {
        path_buf.clear();

        let bbox = {
            let mut builder = Builder(&mut path_buf);
            match face.outline_glyph(GlyphId(id), &mut builder) {
                Some(v) => v,
                None => continue,
            }
        };

        path_buf.clear();

        let width = bbox.x_max as f32 - bbox.x_min as f32;
        let height = bbox.y_max as f32 - bbox.y_min as f32;
        let max_dim = width.max(height);
        let padding_x = (max_dim - width) / 2.0;
        let padding_y = (max_dim - height) / 2.0;

        {
            let mut remapped_builder = RemappedBuilder {
                buf: &mut path_buf,
                input_rect: Rect {
                    start: Vector {
                        x: bbox.x_min as f32 - padding_x,
                        y: bbox.y_min as f32 - padding_y,
                    },
                    end: Vector {
                        x: bbox.x_max as f32 + padding_x,
                        y: bbox.y_max as f32 + padding_y,
                    },
                },
                output_rect: Vector {
                    x: output_dim,
                    y: output_dim,
                }
                .into_absolute_rect(),
            };

            match face.outline_glyph(GlyphId(id), &mut remapped_builder) {
                Some(_) => {}
                None => continue,
            };
        }

        if !path_buf.is_empty() {
            path_buf.pop();
        }

        svgs.push((
            face.glyph_name(GlyphId(id)).unwrap().to_owned(),
            format!(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {output_dim} {output_dim}">
  <path d="{}" />
</svg>"#, path_buf),
        ));
    }

    svgs
}

pub fn scrap_font_icons<T: IntoUrl + Send + 'static>(
    css_url: T,
    depth: usize,
) -> BoxFuture<'static, AnyResult<Vec<(String, String)>>> {
    async move {
        let mut output = Vec::new();
        let css_url = css_url.into_url()?;

        let css_file = reqwest::get(css_url.clone())
            .await?
            .error_for_status()?
            .text()
            .await?;
        let mut webfont_urls = Vec::new();
        for font_face_capture in FONT_FACE_REGEX.captures_iter(&css_file) {
            let font_face = &font_face_capture[0];
            for src_capture in FONT_FACE_URL_REGEX.captures_iter(font_face) {
                let src = src_capture[0].to_string();
                let url = if src.starts_with("url(\"") && src.ends_with("\")") {
                    src.split("(\"")
                        .nth(1)
                        .unwrap()
                        .split("\")")
                        .next()
                        .unwrap()
                        .to_string()
                } else {
                    src.split('(')
                        .nth(1)
                        .unwrap()
                        .split(')')
                        .next()
                        .unwrap()
                        .to_string()
                };
                let ext = ext(&url).unwrap_or("".to_string());
                if !webfont_urls.contains(&url) && ext == "ttf" {
                    webfont_urls.push(url.clone());
                }
            }
        }

        for webfont_url in webfont_urls.into_iter() {
            let url = match css_url.join(&webfont_url) {
                Ok(v) => v,
                Err(_) => continue,
            };

            let bytes = reqwest::get(url).await?.bytes().await?;
            let mut font = Face::parse(&bytes, 0).unwrap();
            for (name, svg) in font_to_svg(&mut font) {
                output.push((
                    format!("{}-{}", normalize(&webfont_url), name),
                    svg,
                ));
            }
        }

        if depth > 0 {
            for css_import_capture in CSS_IMPORT_REGEX.captures_iter(&css_file)
            {
                let imported_css_url = css_import_capture
                    .get(1)
                    .or_else(|| css_import_capture.get(2))
                    .or_else(|| css_import_capture.get(3))
                    .unwrap()
                    .as_str();
                let absolute_url = match css_url.join(imported_css_url) {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                if absolute_url.host_str() == css_url.host_str()
                    && absolute_url.path() != css_url.path()
                {
                    let scraped_font_icons =
                        match scrap_font_icons(absolute_url, depth - 1).await {
                            Ok(v) => v,
                            Err(_) => continue,
                        };
                    output.extend(scraped_font_icons);
                }
            }
        }

        Ok(output)
    }
    .boxed()
}
