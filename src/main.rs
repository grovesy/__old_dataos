use crate::core::urn::Urn;
use crate::core::content::Content;
use crate::core::content::ContentHeader;

pub mod core;
fn main() {

    // A blob of data
    let trade_str = String::from("I am trade 12345}");
    let data = trade_str.as_bytes();

    // the trade identifier
    let trade_id: String = String::from("12345");

    // A Schema 'data' urn
    let data_urn = Urn::new("data://rates.trade/ir-swap");

    // the urn of the business object, data://rates.trade/ir-swap/12345'
    let urn = Urn::create_urn(&data_urn, &trade_id);
 
    // resource://some.app/cdef123412312 .. the app:// is implicit in the os
    // i.e. we cannot run this code without having intiates the library with 
    // this apps app:// identifer
    let resource_urn: Urn = Urn::create_resource_urn(&data_urn, data);
 
    // Headers for the conten
    let header = ContentHeader {
        content_type: String::from("application/json") 
    };

    // the content record
    let content = Content { resource_urn, urn, data, header };

    // Write the content
    content.put();
}
