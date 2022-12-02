pub struct Urn {
    pub value: String
}

impl Urn {
    pub fn new(value: &str) -> Self {
        Urn {
            value: value.to_string()
        }
    }

    pub fn create_urn(_data_urn: &Urn, _id: &String) -> Self {
        Self::new("foobar")
    }
    
    pub fn create_resource_urn(_data_urn: &Urn, _data: &[u8]) -> Self {
        Self::new("resource://wibble")
    }
    
    pub fn get(self: &Self) {
    }
}