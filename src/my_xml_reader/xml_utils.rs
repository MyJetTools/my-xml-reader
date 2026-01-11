pub const OPEN_TAG_TOKEN: u8 = b'<';
pub const CLOSE_TAG_TOKEN: u8 = b'>';
pub const OPEN_HEADER_TOKEN: &'static [u8] = "<?".as_bytes();
pub const CLOSE_HEADER_TOKEN: &[u8] = "?>".as_bytes();
use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    static ref XML_ESC: HashMap<&'static str, &'static str> = [
        ("&quot;", "\""),
        ("&apos;", "'"),
        ("&lt;", "<"),
        ("&gt;", ">"),
        ("&amp;", "&")
    ]
    .iter()
    .copied()
    .collect();
}

pub fn has_special_symbol(xml_string: &str) -> bool {
    for key in XML_ESC.keys() {
        if xml_string.find(key).is_some() {
            return true;
        }
    }

    return false;
}

pub fn decode_xml_string(xml_string: &str) -> String {
    let mut result: String = xml_string.to_string();

    for (key, value) in XML_ESC.iter() {
        result = result.replace(key, value);
    }

    return result;
}

fn skip_xml_header<'t>(xml: &'t [u8], start_pos: usize) -> Result<usize, String> {
    // Work with a slice that starts at the first '<' we saw, but keep track of
    // the absolute offset so we return positions relative to the original
    // buffer.
    let xml_slice = &xml[start_pos..];

    if !xml_slice.starts_with(OPEN_HEADER_TOKEN) {
        return Ok(start_pos);
    }

    let close_header_pos = find_next_token_ext(xml_slice, CLOSE_HEADER_TOKEN, 0);

    match close_header_pos {
        Some(end_header_pos) => {
            let pos = find_next_token(
                xml_slice,
                OPEN_TAG_TOKEN,
                end_header_pos + CLOSE_HEADER_TOKEN.len(),
            );

            match pos {
                Some(pos) => Ok(start_pos + pos),
                None => Err("Can not find root TAG after header Node".to_string()),
            }
        }
        None => {
            return Err("Can not find close header Node".to_string());
        }
    }
}

pub fn init_pos_start<'t>(xml: &'t [u8]) -> Result<usize, String> {
    let pos = find_next_token(xml, OPEN_TAG_TOKEN, 0);

    return match pos {
        Some(pos) => skip_xml_header(xml, pos),
        None => Err("Can not fine first TAG or XML Header such as '<'".to_string()),
    };
}

pub fn find_next_token<'t>(xml: &'t [u8], token_to_find: u8, start_pos: usize) -> Option<usize> {
    for pos in start_pos..xml.len() {
        if xml[pos] == token_to_find {
            return Some(pos);
        }
    }

    None
}

pub fn extract_tag_name<'t>(node_tag: &'t [u8]) -> &'t [u8] {
    let mut loop_start: usize = 1;

    if node_tag[1] == b'/' {
        loop_start = 2;
    }

    for i in loop_start..node_tag.len() - 1 {
        if node_tag[i] <= 32 || node_tag[i] == b'/' {
            return &node_tag[loop_start..i];
        }
    }

    return &node_tag[loop_start..node_tag.len() - 1];
}

fn find_next_token_ext<'t>(xml: &'t [u8], token_to_find: &[u8], start_pos: usize) -> Option<usize> {
    if token_to_find.len() == 2 {
        for pos in start_pos..xml.len() - 1 {
            if xml[pos] == token_to_find[0] && xml[pos + 1] == token_to_find[1] {
                return Some(pos);
            }
        }
        return None;
    }

    for pos in start_pos..xml.len() {
        let mut all_match = true;

        for sub_pos in 0..token_to_find.len() {
            if xml[pos + sub_pos] != token_to_find[sub_pos] {
                all_match = false;
                break;
            }
        }

        if all_match {
            return Some(pos);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_node_name() {
        let xml_src = r#"<RootNode/>"#;

        let node_name = extract_tag_name(xml_src.as_bytes());

        assert_eq!("RootNode", std::str::from_utf8(node_name).unwrap());
    }

    #[test]
    fn test_extract_node_name_ver2() {
        let xml_src = r#"<RootNode />"#;

        let node_name = extract_tag_name(xml_src.as_bytes());

        assert_eq!("RootNode", std::str::from_utf8(node_name).unwrap());
    }

    #[test]
    fn test_extract_node_name_ver3() {
        let xml_src = r#"<RootNode>"#;

        let node_name = extract_tag_name(xml_src.as_bytes());

        assert_eq!("RootNode", std::str::from_utf8(node_name).unwrap());
    }

    #[test]
    fn test_extract_node_name_ver4() {
        let xml_src = r#"</RootNode>"#;

        let node_name = extract_tag_name(xml_src.as_bytes());

        assert_eq!("RootNode", std::str::from_utf8(node_name).unwrap());
    }

    #[test]
    fn test_init_pos_start_with_header_and_leading_space() {
        let xml_src = "   <?xml version=\"1.0\"?><Root/>";

        let pos = init_pos_start(xml_src.as_bytes()).unwrap();

        assert_eq!("<Root/>", &xml_src[pos..]);
    }

    #[test]
    fn test_init_pos_start_with_header_and_bom() {
        let xml_src = "\u{feff}<?xml version=\"1.0\"?><Root></Root>";
        let bytes = xml_src.as_bytes();

        let pos = init_pos_start(bytes).unwrap();

        assert_eq!("<Root></Root>", std::str::from_utf8(&bytes[pos..]).unwrap());
    }

    #[test]
    fn test_init_pos_start_without_header_and_leading_space() {
        let xml_src = "   <Root></Root>";
        let pos = init_pos_start(xml_src.as_bytes()).unwrap();

        assert_eq!("<Root></Root>", &xml_src[pos..]);
    }

    #[test]
    fn test_init_pos_start_with_header_and_gap_before_root() {
        let xml_src = "<?xml version=\"1.0\"?>   <Root></Root>";
        let pos = init_pos_start(xml_src.as_bytes()).unwrap();

        assert_eq!("<Root></Root>", &xml_src[pos..]);
    }
}
