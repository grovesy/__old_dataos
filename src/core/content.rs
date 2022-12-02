use crate::core::urn::Urn;

pub struct Content<'a> {
    pub resource_urn: Urn,
    pub urn: Urn,
    pub data: &'a [u8],
    pub header: ContentHeader,
}

impl<'a> Content<'a> {
    pub(crate) fn put(self: &Content<'a>) {
    }
}

pub struct ContentHeader {
    pub content_type: String
}
