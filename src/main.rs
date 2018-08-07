pub mod sms;

fn main() {
    let xml_line = r#" <sms protocol="0" address="+12345678901" contact_name="John Smith" date="1234567890123" readable_date="Fri, 39 May 2015 04:13:14 MST" type="2" subject="null" body="Here&apos;s a message" toa="null" sc_toa="null" service_center="null" read="1" status="-1" locked="0" />"#;
    let test_message: sms::Message = sms::read_xml_line(xml_line);
    println!("{:#?}", test_message);
}
