mod my_xml_node;
mod my_xml_reader;
mod xml_tag_info;
pub mod xml_utils;

pub use my_xml_node::MyXmlNode;
pub use xml_tag_info::XmlTagInfo;
pub use xml_tag_info::XmlTagType;

pub use my_xml_reader::MyXmlReader;
pub use my_xml_reader::OpenedNode;
mod attributes_iterator;
pub use attributes_iterator::*;
