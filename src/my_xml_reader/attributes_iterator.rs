pub struct AttributesIterator<'s> {
    data: &'s [u8],
    pos: usize,
}

impl<'s> AttributesIterator<'s> {
    pub fn new(data: &'s [u8]) -> Self {
        Self {
            data,
            pos: find_start_of_attribute(data),
        }
    }

    pub fn get_next(&mut self) -> Option<(&'s str, &'s str)> {
        let pos = self.pos;

        if pos >= self.data.len() {
            return None;
        }

        if self.data[pos] == b'/' {
            return None;
        }

        if self.data[pos] == b'>' {
            return None;
        }

        let result = extract_attr_and_value(&self.data[pos..]).unwrap();

        self.pos = pos + result.len;

        result.key_value
    }
}

fn find_start_of_attribute(data: &[u8]) -> usize {
    let mut found_space = false;
    for (i, b) in data.iter().enumerate() {
        let b = *b;

        if found_space {
            match b {
                b'/' => {
                    return data.len();
                }
                b'>' => {
                    return data.len();
                }
                b' ' => {}
                _ => return i,
            }
        } else {
            match b {
                b' ' => {
                    found_space = true;
                }
                b'/' => {
                    return data.len();
                }
                b'>' => {
                    return data.len();
                }
                _ => {}
            }
        }
    }

    data.len()
}

fn extract_attr_and_value<'s>(src: &'s [u8]) -> Result<FoundTagData<'s>, String> {
    let mut attr_start = None;
    let mut eq_pos = None;
    let mut end_pos = src.len();

    let mut quote_amount = 0;

    let mut esc_charged = false;

    for (i, b) in src.iter().enumerate() {
        let b = *b;
        if eq_pos.is_none() {
            if attr_start.is_none() {
                if b == b' ' {
                    continue;
                }

                attr_start = Some(i);
            }
            match b {
                b'=' => eq_pos = Some(i),
                b'>' => {
                    return Ok(FoundTagData {
                        key_value: None,
                        len: i,
                    });
                }
                b'/' => {
                    return Ok(FoundTagData {
                        key_value: None,
                        len: i,
                    });
                }
                _ => {}
            }
        } else {
            if esc_charged {
                esc_charged = false;
                continue;
            }

            match b {
                b'"' => {
                    quote_amount += 1;
                }
                b'\\' => {
                    esc_charged = true;
                }
                _ => {}
            }

            if quote_amount == 2 {
                end_pos = i;
                break;
            }
        }
    }

    let Some(eq_pos) = eq_pos else {
        return Err("Can not read attribute. Eq position is not found".to_string());
    };

    let attr_start = attr_start.unwrap();

    let result = FoundTagData {
        key_value: Some((
            std::str::from_utf8(&src[attr_start..eq_pos]).unwrap(),
            std::str::from_utf8(&src[eq_pos + 2..end_pos]).unwrap(),
        )),
        len: end_pos + 1,
    };

    Ok(result)
}

pub struct FoundTagData<'s> {
    pub key_value: Option<(&'s str, &'s str)>,
    pub len: usize,
}
#[cfg(test)]
mod tests {
    use crate::my_xml_reader::AttributesIterator;

    #[test]
    fn test_tags_iterator_with_no_attrs() {
        let xml = "<test></test>";

        let mut attrs_iterator = AttributesIterator::new(xml.as_bytes());
        let next = attrs_iterator.get_next();
        assert!(next.is_none());
    }

    #[test]
    fn test_tags_iterator_with_no_attrs_case_2() {
        let xml = "<test/>";

        let mut attrs_iterator = AttributesIterator::new(xml.as_bytes());
        let next = attrs_iterator.get_next();
        assert!(next.is_none());
    }

    #[test]
    fn test_tags_iterator_with_no_attrs_case_3() {
        let xml = "<test />";

        let mut attrs_iterator = AttributesIterator::new(xml.as_bytes());
        let next = attrs_iterator.get_next();
        assert!(next.is_none());
    }

    #[test]
    fn test_tags_iterator_with_single_attribute() {
        let xml = "<test attr1=\"value1\" attr2=\"value2\"/>";

        let mut attrs_iterator = AttributesIterator::new(xml.as_bytes());
        let next = attrs_iterator.get_next().unwrap();

        assert_eq!(next.0, "attr1");
        assert_eq!(next.1, "value1");

        let next = attrs_iterator.get_next().unwrap();

        assert_eq!(next.0, "attr2");
        assert_eq!(next.1, "value2");
    }
}
