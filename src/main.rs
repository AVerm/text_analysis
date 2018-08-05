fn main() {
    let xml_line = r#" <sms protocol="0" address="+12345678901" contact_name="John Smith" date="1234567890123" readable_date="Fri, 39 May 2015 04:13:14 MST" type="2" subject="null" body="Here&apos;s a message" toa="null" sc_toa="null" service_center="null" read="1" status="-1" locked="0" />"#;
    let test_message: SmsMessage = read_xml_line(xml_line);
    println!("{:#?}", test_message);
}

#[derive(Debug)]
struct SmsMessage<'a> {
    protocol: u32,
    address: &'a str,
    contact_name: &'a str,
    date: i64,
    readable_date: &'a str,
    type_: u32,
    subject: &'a str,
    body: String,
    toa: &'a str,
    sc_toa: &'a str,
    service_center: &'a str,
    read: bool,
    status: i32,
    locked: bool,
}

/// Turns an xml line into a SmsMessage struct, assuming the fields go in a certain order
/// This is the format of a typical line, with example values filled in
/// <sms protocol="0" address="+12345678901" contact_name="John Smith" date="1234567890123" readable_date="Fri, 39 May 2015 04:13:14 MST" type="2" subject="null" body="Here&apos;s a message" toa="null" sc_toa="null" service_center="null" read="1" status="-1" locked="0" />
fn read_xml_line<'a>(line: &'a str) -> SmsMessage<'a> {
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
    SmsMessage {protocol, address, contact_name, date, readable_date, type_, subject, body, toa, sc_toa, service_center, read, status, locked}
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
    let mut clean: String = "".to_string();
    clean.reserve(unclean.chars().count());

    //Body start
    println!("{}", unclean.chars().count());
    for (num, first_index) in unclean.chars().enumerate() {
        println!("{}", unclean.get(first_index).unwrap());
    }
    /*
    let mut unclean_chars = unclean.chars();
    for unclean_char in unclean_chars {
        println!("{}", unclean_char);
        if unclean_char == '&' {
            let mut buffer: String = "".to_string();
            let current = unclean_chars.next();
            while current.unwrap() != ';' {
                buffer.push(current.unwrap());
                current = unclean_chars.next();
            }
            clean.push(match &*buffer {
                "apos" => '\'',
                "amp"  => '&',
                "quot" => '\"',
                "lt"   => '<',
                "gt"   => '>',
                "#10:" => ' ', // I think it is ASCII 10 (Linefeed)
                _      => ' ',
            });
        }
    }
    */
    //Body end

    clean.shrink_to_fit();
    clean
}


/*
let clean_body: String = "";
for first_index in 0..unclean_body.len(){
    let this_character: String;
    if unclean_body.get(first_index) == '&' {
        for second_index in first_index+1..unclean_body.len() {
            let character_buffer: String= "";
            if unclean_body.get(second_index) != ';' {
                character_buffer += unclean_body.get(second_index).unwrap();
            }

            let this_character = match character_buffer {
                "apos" => '\'',
                "amp" => '&',
                "quot" => '\"',
                "lt" => '<',
                "gt" => '>',
                "#10:" => ' ', // I think it is ASCII 10 (Linefeed)
                _ => ' ',
            }
        }
    }
    else {
        this_character = unclean_body.get(first_index).unwrap();
    }
    clean_body.push_str(&this_character);
}
clean_body
*/
