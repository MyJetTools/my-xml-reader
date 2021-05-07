use super::XmlTagInfo;

pub struct MyXmlNode<'t> {
    pub xml: &'t [u8],
    pub open_node: XmlTagInfo<'t>,
    pub close_node: Option<XmlTagInfo<'t>>,
}

impl<'t> MyXmlNode<'t> {
    pub fn get_node_name(&self) -> &'t str {
        self.open_node.name
    }

    pub fn get_inner_content(&self) -> Option<&'t [u8]> {
        let result = match &self.close_node {
            Some(close_node) => Some(&self.xml[self.open_node.end_pos + 1..close_node.start_pos]),
            None => None,
        };

        result
    }

    pub fn get_value(&self) -> Option<String> {
        let inner_content = self.get_inner_content()?;

        let value = std::str::from_utf8(inner_content).unwrap();

        if super::xml_utils::has_special_symbol(value) {
            return Some(super::xml_utils::decode_xml_string(value));
        }

        return Some(value.to_string());
    }

    pub fn get_xml(&self) -> &'t str {
        let xml = match &self.close_node {
            Some(close_node) => &self.xml[self.open_node.start_pos..close_node.end_pos + 1],
            None => &self.xml[self.open_node.start_pos..self.open_node.end_pos + 1],
        };

        return std::str::from_utf8(xml).unwrap();
    }
}
