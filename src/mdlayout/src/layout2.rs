use crate::graphics_engine::renderer::*;
use crate::style::values::*;
use crate::style::{style_for_element, StyleSet, ComputedValues};

use crate::primitives::{CssPx, RGBA};

// for dom
use crate::dom::*;

use std::str;
use std::cell::RefCell;
use crate::graphics_engine::shaped_segment::ShapedSegment;

pub struct Context<'a> {
    pub document: &'a Document,
    pub author_styles: &'a StyleSet,
}

pub enum InlineBoxContent {
    Space,
    Linebreak,
    Text(RefCell<String>),
}

struct InlineBox {
    pub content: InlineBoxContent,
    pub width: f64,
    pub linespacing: f64,
    pub font: Font,
    pub color: RGBA,
}


fn make_text_boxes(
    boxes: &mut Vec<InlineBox>,
    text: &str,
    font: &Font,
    color: RGBA,
) {
    let fm = font.font_metrics();

    // add a starting whitespace box if none exists yet
    if text.starts_with(|x: char| x.is_ascii_whitespace()) {
        maybe_add_space(boxes, &fm, font);
    }

    for word in text.split_ascii_whitespace() {
        // push word, then space

        let mut s = ShapedSegment::shape(word, font.clone()).unwrap();
        let w = s.get_advance_width().unwrap();
        let b = InlineBox {
            content: InlineBoxContent::Text(RefCell::new(word.to_string())),
            width: (w.get() as f64) / 96.0,
            linespacing: fm.linespacing,
            font: font.clone(),
            color: color,
        };
        boxes.push(b);
        add_space(boxes, &fm, font);
    }

    // remove final space if string doesn't end with whitespace
    if !text.ends_with(|x: char| x.is_ascii_whitespace()) {
        maybe_remove_space(boxes);
    }
}

/// Unconditionally add a space box
fn add_space(boxes: &mut Vec<InlineBox>, fm: &FontMetrics, font: &Font) {
    let b = InlineBox {
        content: InlineBoxContent::Space,
        width: fm.space_width,
        linespacing: fm.linespacing,
        font: font.clone(),
        color: RGBA(0, 0, 0, 0),
    };

    boxes.push(b);
}

/// Add space only if current box list doesn't end in a space.
/// Never adds a space to an empty box list or after a linebreak.
fn maybe_add_space(boxes: &mut Vec<InlineBox>, fm: &FontMetrics, font: &Font) {
    if let Some(b) = boxes.last() {
        match b.content {
            InlineBoxContent::Space => {},
            InlineBoxContent::Linebreak => {},
            _ => add_space(boxes, fm, font),
        }
    }
}

/// Remove last box if it is a space box
fn maybe_remove_space(boxes: &mut Vec<InlineBox>) {
    if let Some(b) = boxes.last() {
        if let InlineBoxContent::Space = b.content {
            boxes.pop();
        }
    }
}

/// Add a newline box. First removes a last space if it exists.
fn add_newline(boxes: &mut Vec<InlineBox>, font: &Font) {
    let fm = font.font_metrics();

    maybe_remove_space(boxes);

    let b = InlineBox {
        content: InlineBoxContent::Linebreak,
        width: 0.0,
        linespacing: fm.linespacing,
        font: font.clone(),
        color: RGBA(0, 0, 0, 0),
    };
    boxes.push(b);
}

fn apply_style_attributes(style: &ComputedValues, gc: &GContext) -> GContext {
    let mut gc_new = gc.clone();
    gc_new.set_color(style.color.color.into());
    gc_new.set_fontstyle(style.font.font_style);
    gc_new.set_fontweight(style.font.font_weight);
    gc_new.set_fontsize(style.font.font_size.0.px as f64);
    let family = match &style.font.font_family {
        FontFamily::GenericSans => "sans",
        FontFamily::GenericSerif => "serif",
        FontFamily::GenericMonospace => "mono",
        FontFamily::FamilyName(ref s) => s.as_str(),
        _ => "sans", // use sans for Fantasy and Cursive
    };
    gc_new.set_fontfamily(family);
    gc_new
}

fn retrieve_font(style: &ComputedValues, fm: &FontManager) -> Font {
    let family = match &style.font.font_family {
        FontFamily::GenericSans => "sans",
        FontFamily::GenericSerif => "serif",
        FontFamily::GenericMonospace => "mono",
        FontFamily::FamilyName(ref s) => s.as_str(),
        _ => "sans", // use sans for Fantasy and Cursive
    };

    fm.new_font(
        family,
        style.font.font_style,
        style.font.font_weight,
        style.font.font_size.0
    )
}

fn process_node<'dom>(
    boxes: &mut Vec<InlineBox>,
    node_id: NodeId,
    parent_element_style: &ComputedValues,
    context: &'dom Context,
    fm: &FontManager
) {
    let style = style_for_element(
        context.author_styles,
        context.document,
        node_id,
        Some(parent_element_style),
    );

    let font = retrieve_font(&style, fm);

    let node = &context.document[node_id];
    match node.data {
        NodeData::Element(ref elt) => {
            match &elt.name.local {
                &local_name!("br") => add_newline(boxes, &font),
                _ => {},
            }
        },
        NodeData::Text{ref contents} => {
            make_text_boxes(boxes, contents, &font, style.color.color.into());
        },
        _ => {},
    }

    if let Some(child_id) = node.first_child {
        for nid in context.document.node_and_following_siblings(child_id) {
            process_node(
                boxes,
                nid,
                &style,
                context,
                fm
            );
        }
    }
}

fn render_inline_boxes(inline_boxes: &Vec<InlineBox>, rdev: &mut RenderDevice) {
    let x0 = 0.2;
    let y0 = 0.5;
    let mut x = 0.0;
    let mut y = 0.0;
    for b in inline_boxes {
        match &b.content {
            InlineBoxContent::Space => {
                x += b.width;
            },
            InlineBoxContent::Linebreak=> {
                x = 0.0;
                y += b.linespacing;
            },
            InlineBoxContent::Text(word) => {
                rdev.draw_text(&word.borrow(), x0 + x, y0 + y, &b.font, b.color);
                x += b.width;
            }
        }
    }
}

pub fn render_html(text_input: &str, css_input: &str, mut rdev: RenderDevice) {
    let mut inline_boxes: Vec<InlineBox> = Vec::new();
    let document = Document::parse_html(text_input.as_bytes());
    let author_styles = &document.parse_stylesheets(Some(css_input));
    let context = Context {
        document: &document,
        author_styles,
    };
    let root_element = document.root_element();
    let style = style_for_element(context.author_styles, context.document, root_element, None);
    let fm = rdev.new_font_manager();

    process_node(&mut inline_boxes, document.root_element(), &style, &context, &fm);
    render_inline_boxes(&inline_boxes, &mut rdev);
}