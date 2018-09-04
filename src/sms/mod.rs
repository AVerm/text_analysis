use std::error::Error;

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

pub struct Contact {
    pub address: String,
    pub contact_name: String,
    pub count_to: usize,
    pub length_to: usize,
    pub count_from: usize,
    pub length_from: usize,
}

impl<'m> Message<'m> {
    /// Turns an xml line into a sms::Message struct, assuming the fields go in a certain order
    /// This is the format of a typical line, with example values filled in
    /// <sms protocol="0" address="+12345678901" contact_name="John Smith" date="1234567890123" readable_date="Fri, 39 May 2015 04:13:14 MST" type="2" subject="null" body="Here&apos;s a message" toa="null" sc_toa="null" service_center="null" read="1" status="-1" locked="0" />
    pub fn read_from_xml<'a>(line: &'a str) -> Result<Message<'a>, Box<Error>> {
        let mut fields = line.trim().trim_left_matches("<sms") // Now it is just the fields and the close tag
            .split('"')
            .map(|field| field.trim()) // Breaks up so that every field name is followed by its contents
            .collect::<Vec<&str>>();

        fn get_field<'a>(fields: &mut Vec<&'a str>, label: &str) -> Result<&'a str, Box<Error>>
        {
            let index = (&fields).iter().position(|ref field| label == field.trim_right_matches('='));
            Ok(
                fields[
                    index.ok_or_else(
                        || format!("Field {} not found", label)
                    )? + 1
                ]
            )
        }

        let protocol       = get_field(&mut fields, "protocol")?.parse::<u32>()?;
        let address        = get_field(&mut fields, "address")?;
        let contact_name   = get_field(&mut fields, "contact_name")?;
        let date           = get_field(&mut fields, "date")?.parse()?;
        let readable_date  = get_field(&mut fields, "readable_date")?;
        let type_          = get_field(&mut fields, "type")?.parse()?;
        let subject        = get_field(&mut fields, "subject")?;
        let body           = desanitize(get_field(&mut fields, "body")?); // TODO: There may be an error, needs to be fixed
        let toa            = get_field(&mut fields, "toa")?;
        let sc_toa         = get_field(&mut fields, "sc_toa")?;
        let service_center = get_field(&mut fields, "service_center")?;
        let read           = get_field(&mut fields, "read")?.parse::<i32>()?==1;
        let status         = get_field(&mut fields, "status")?.parse::<i32>()?;
        let locked         = get_field(&mut fields, "locked")?.parse::<i32>()?==1;
        Ok(Message {protocol, address, contact_name, date, readable_date, type_, subject, body, toa, sc_toa, service_center, read, status, locked})
    }
}

impl Contact {
    pub fn new(contact_name: &str, address: &str) -> Contact {
        Contact {
            address: address.to_owned(),
            contact_name: contact_name.to_owned(),
            count_to: 0,
            length_to: 0,
            count_from: 0,
            length_from: 0,
        }
    }

    pub fn record(self, message: &Message) -> Contact {
                Contact {
                    address:      self.address,
                    contact_name: self.contact_name,
                    count_to:     self.count_to    + if message.type_ == 2 {1} else {0},
                    length_to:    self.length_to   + if message.type_ == 2 {message.body.chars().count()} else {0},
                    count_from:   self.count_from  + if message.type_ == 1 {1} else {0},
                    length_from:  self.length_from + if message.type_ == 1 {message.body.chars().count()} else {0},
                }
    }
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
fn desanitize(unclean: &str) -> String {
    // Create a String to hold the cleaned body
    let mut clean: String = "".to_string();
    // Reserve as many characters as needed to hold the unclean. No characters
    // will expand, but escaped characters will shrink, so this will mean no
    // extra allocations to slow down additions.
    clean.reserve(unclean.chars().count());
    //Body start
    let mut buf = "".to_string();
    for current in unclean.chars() {
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
    }
    clean.shrink_to_fit();
    clean
}

#[cfg(test)]
mod test {
    use sms::message::parse_body;
    #[test]
    fn test_desanitization() {
        assert_eq!(parse_body("&apos;&amp;&quot;&lt;&gt;"), "\'&\"<>");
    }
}
