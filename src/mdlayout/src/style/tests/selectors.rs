// A complete test of parsing some html and css and applying the appropriate rules
// to specific elements

use crate::{
    dom::{Document, NodeId, NodeData},
    style::{StyleSet, style_for_element}
};

use cssparser::RGBA;

fn process_node<'dom>(node_id: NodeId, author_styles: &'dom StyleSet, document: &'dom Document,
) {
    let style = style_for_element(author_styles, document, node_id, None);

    let node = &document[node_id];
    match node.data {
        NodeData::Element(ref elt) => {
            println!("element: {:?}", elt.name.local);
        },
        NodeData::Text{ref contents} => {
            println!("text: {}", contents);
        },
        _ => {
            println!("other");
        },
    }

    println!("color: {:?}", &style.color.color);
    println!("background color: {:?}", &style.background.background_color);


    if let Some(child_id) = node.first_child {
        for (number, nid) in document.node_and_following_siblings(child_id).enumerate() {
            println!("nesting level: {}", number);
            process_node(nid, author_styles, document);
        }
    }
}

// validate that a given node is an element and has the expected type
macro_rules! validate_element_type {
        ($node_id:expr, $element_name:tt, $document:expr) => {
            let node = &$document[$node_id];
            if let NodeData::Element(ref elt) = node.data {
                assert_eq!(elt.name.local, local_name!($element_name));
            } else {
                assert!(false);
            }
        }
    }

// validate that a given node is text and has the expected contents
macro_rules! validate_text {
        ($node_id:expr, $contents:expr, $document:expr) => {
            let node = &$document[$node_id];
            if let NodeData::Text{ref contents} = node.data {
                assert_eq!(contents, $contents);
            } else {
                assert!(false);
            }
        }
    }

// validate that the style for the element has the expected color=
macro_rules! validate_color {
        ($node_id:expr, $color:expr, $document:expr, $author_styles:expr) => {
            let style = style_for_element($author_styles, &$document, $node_id, None);
            assert_eq!(style.color.color, $color);
        }
    }

#[test]
fn selectors() {
    let text_input = "<body><p>Hello world</p></body>";
    let css_input = r#"
        p {color: red;}
        body {color: green;}
    "#;

    let document = Document::parse_html(text_input.as_bytes());
    let author_styles = &document.parse_stylesheets(Some(css_input));
    process_node(document.root_element(), author_styles, &document);


    let node_id = document.root_element();
    validate_element_type!(node_id, "html", document);
    // default color is black
    validate_color!(node_id, RGBA::new(0, 0, 0, 255), document, author_styles);

    let node_id = document[node_id].first_child.unwrap();
    validate_element_type!(node_id, "head", document);

    let node_id = document[node_id].next_sibling.unwrap();
    validate_element_type!(node_id, "body", document);
    validate_color!(node_id, RGBA::new(0, 128, 0, 255), document, author_styles);

    let node_id = document[node_id].first_child.unwrap();
    validate_element_type!(node_id, "p", document);
    validate_color!(node_id, RGBA::new(255, 0, 0, 255), document, author_styles);

    let node_id = document[node_id].first_child.unwrap();
    validate_text!(node_id, "Hello world", document);
    // we're not propagating styles in this test example, so this should be default again
    validate_color!(node_id, RGBA::new(0, 0, 0, 255), document, author_styles);
}