use crate::core::urn::Urn;

pub struct BusinessMeta {
    pub urn: Urn,                       // friendly urn . data://securities/bond/isin-34242342
    pub data_urn: Urn,                  // the schema - data://some.domain/some-model
    pub content_urn: Urn,               // the hash urn resource://some.app/cdef1234
    pub system_valid: TemporalRecord,
    pub business_valid: TemporalRecord,
}

pub struct TemporalRecord {
    pub from: u64,
    pub to: u64,
}

pub struct Xref {
}

pub struct ContentIndex {
    pub urn: Urn,
    pub index: std::collections::HashMap<String, String>,
}


