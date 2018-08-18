#[derive(Debug)]
pub struct Message<'a> {
    pub protocol: u32,
    pub address: &'a str, /// The number the message was sent to or from
    pub contact_name: &'a str, /// The name of the contact, null if not present
    pub date: i64, /// Seconds since 1970-01-01 00:00:00 UTC but the last 3 digits appear to be decimal seconds
    pub readable_date: &'a str, /// A human readable date such as "Sat, 18 Aug 2018 12:57:13 MST"
    pub type_: u32, /// Either 1 or 2. A message sent from self to other is 2, a message sent from other to self is 1 (recieved = 1, sent = 2)
    pub subject: &'a str,
    pub body: String, /// The actual message sent
    pub toa: &'a str,
    pub sc_toa: &'a str,
    pub service_center: &'a str,
    pub read: bool,
    pub status: i32,
    pub locked: bool,
}

#[derive(Debug)]
pub struct Contact {
    pub address: String,
    pub contact_name: String,
    pub count_to: usize,
    pub length_to: usize,
    pub count_from: usize,
    pub length_from: usize,
}

/// Turns an xml line into a sms::Message struct, assuming the fields go in a certain order
/// This is the format of a typical line, with example values filled in
/// <sms protocol="0" address="+12345678901" contact_name="John Smith" date="1234567890123" readable_date="Fri, 39 May 2015 04:13:14 MST" type="2" subject="null" body="Here&apos;s a message" toa="null" sc_toa="null" service_center="null" read="1" status="-1" locked="0" />
pub fn read_xml_line<'a>(line: &'a str) -> Message<'a> {
    let line_as_vec = line.split("\"").collect::<Vec <&str>>();
    let protocol       = line_as_vec[1].parse().unwrap();
    let address        = line_as_vec[3];
    let contact_name   = line_as_vec[5];
    let date           = line_as_vec[7].parse().unwrap();
    let readable_date  = line_as_vec[9];
    let type_          = line_as_vec[11].parse().unwrap();
    let subject        = line_as_vec[13];
    let body           = parse_body(line_as_vec[15]);
    let toa            = line_as_vec[17];
    let sc_toa         = line_as_vec[19];
    let service_center = line_as_vec[21];
    let read           = line_as_vec[23].parse::<i32>().unwrap()==1;
    let status         = line_as_vec[25].parse().unwrap();
    let locked         = line_as_vec[27].parse::<i32>().unwrap()==1;
    Message {protocol, address, contact_name, date, readable_date, type_, subject, body, toa, sc_toa, service_center, read, status, locked}
}

/// Cleans a message body, desanitizing it
/// This means changing numeric character references to their plain representation
/// &apos; --> '
/// &amp;  --> &
/// &quot; --> "
/// &lt;   --> <
/// &gt;   --> >
/// TODO: Add support for generic numerical references (&#10:; --> Character code 10)
/// assert_eq!(parse_body("&apos;&amp;&quot;&lt;&gt;"), "\'&\"<>");
fn parse_body<'a>(unclean: &'a str) -> String {
    // Create a String to hold the cleaned body
    let mut clean: String = "".to_string();
    // Reserve as many characters as needed to hold the unclean. No characters
    // will expand, but escaped characters will shrink, so this will mean no
    // extra allocations to slow down additions.
    clean.reserve(unclean.chars().count());
    //Body start
    let mut chars = unclean.chars();
    let mut buf = "".to_string();
    loop {
        match chars.next() {
            Some(current) => {
                if current == '&' {
                    buf = "&".to_string();
                }
                else if current == ';' {
                    clean.push(
                        match &*buf {
                            "&apos" => '\'',
                            "&amp"  => '&',
                            "&gt"   => '>',
                            "&lt"   => '<',
                            "&quot" => '\"',
                            _       => '?',
                            //_       => buf.tail() TODO
                        }
                    );
                    buf = "".to_string();
                }
                else if !buf.is_empty() {
                    buf.push(current);
                }
                else {
                    clean.push(current);
                }
            },
            None => break,
        }
    }
    clean.shrink_to_fit();
    clean
}

#[cfg(test)]
mod test {
    use sms::parse_body;
    #[test]
    fn test_desanitization() {
        assert_eq!(parse_body("&apos;&amp;&quot;&lt;&gt;"), "\'&\"<>");
    }
}
