# my-xml-reader

Small, forward-only XML helper aimed at lightweight parsing in constrained contexts. It walks a byte slice, exposes tags as `XmlTagInfo`, and lets you pull full nodes as `MyXmlNode` without allocating an entire DOM.

## Features
- Iterate through tags with a tiny stateful `MyXmlReader`.
- Read whole nodes (including nested content) and extract inner text via `MyXmlNode`.
- XPath-lite helpers: `find_the_open_node`, `find_the_node_inside_parent`, and `find_any_of_these_nodes_inside_parent`.
- Attribute iteration with `AttributesIterator`, exposing key/value pairs without extra copies.
- XML escape decoding for common entities (`&quot;`, `&apos;`, `&lt;`, `&gt;`, `&amp;`).
- Handles XML headers and UTF-8 BOM at the start of the buffer.

## Getting started
Add the crate to your `Cargo.toml` (use the path form if you're working in this repo):
```
[dependencies]
my-xml-reader = { path = "../my-xml-reader" }
```

## Quick example
```rust
use my_xml_reader::{MyXmlReader, MyXmlNode};

fn main() -> Result<(), String> {
    let xml = r#"<?xml version="1.0"?>
        <R>
          <A>
            <S1><I id="1">V1</I><I id="2">V2</I></S1>
            <S2>V3</S2>
          </A>
        </R>"#;

    let mut reader = MyXmlReader::from_slice(xml.as_bytes())?;

    // Navigate by segments (very small XPath-like helper).
    let parent = reader.find_the_open_node("R/A")?.unwrap();

    // Pull child nodes by name, streaming forward.
    while let Some(node) = reader.find_any_of_these_nodes_inside_parent(&parent, &["S1", "S2"])? {
        match node.name {
            "S1" => {
                let full = reader.read_the_whole_node(node)?;
                println!("S1 raw: {}", full.get_xml());
            }
            "S2" => {
                let val = reader.read_the_whole_node(node)?.get_value().unwrap();
                println!("S2 value: {val}");
            }
            _ => {}
        }
    }

    Ok(())
}
```

## Working with attributes
```rust
use my_xml_reader::MyXmlReader;

let xml = r#"<Item id="42" name="Widget" enabled="true"/>"#;
let mut reader = MyXmlReader::from_slice(xml.as_bytes())?;
let tag = reader.read_next_tag()?.unwrap(); // first tag is <Item .../>

for (k, v) in tag.iterate_attributes() {
    println!("{k} = {v}");
}
```

## Key types
- `MyXmlReader<'t>`: streaming cursor over the input slice; maintains nesting with `opened_nodes`.
- `XmlTagInfo<'t>`: view of a single tag (`name`, `raw`, `tag_type`, `level`, positions); can iterate attributes.
- `MyXmlNode<'t>`: represents an open/close pair; provides `get_xml()`, `get_inner_content()`, and `get_value()` (decodes escapes).
- `AttributesIterator<'t>`: zero-copy attribute iterator over a tag's raw bytes.
- `XmlTagType`: enum of `Open`, `Close`, `OpenClose`.

## Behavioral notes
- Forward-only: the reader consumes input as you call `read_next_tag`/find functions; it does not rewind.
- Basic XML coverage: no namespace handling, comments, CDATA, or validation. Input should be well-formed for best results.
- Escape decoding is limited to the five common entities; others pass through unchanged.
- Errors are returned as `String`; no custom error type yet.

## Testing
Run the built-in suite:
```
cargo test
```
