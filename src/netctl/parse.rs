use std::str::FromStr;

use nom::{
    IResult, 
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::char,
    sequence::{delimited, separated_pair}
};

use crate::netctl;

// gets the key from the input if it matches a legal alternative
fn parse_key(input: &str) -> IResult<&str, &str> {
    alt((
        tag("Description"),
        tag("Connection"),
        tag("Interface"),
        tag("Security"),
        tag("ESSID"),
    ))(input)
}

// parse until whitespace or comment, but take account escaped chars
// note: comment symbol should technically be preceded by whitespace,
// but netctl seems to be able to parse files that violate this.
fn take_until_ws(input: &str) -> IResult<&str, String> {
    let mut output = String::new();
    let mut skip = false;

    for (i, c) in input.char_indices() {
        if c == '\\' && !skip {
            skip = true;
        } else if (c.is_whitespace() || c == '#') && !skip {
            return Ok((&input[i..], output));
        } else if i == input.len() - 1 {
            output.push(c);
            return Ok((&input[1..], output));
        } else {
            output.push(c);
            skip = false;
        }
    }

    Err(nom::Err::Incomplete(nom::Needed::Unknown))
}

// finds text inside single or double quotes
fn quote_surrounded(input: &str) -> IResult<&str, String> {
    let (fst, snd): (&str, &str) = alt((
        delimited(char('"'), is_not("\""), char('"')),
        delimited(char('\''), is_not("'"), char('\'')),
    ))(input)?;

    Ok((fst, snd.to_string()))
}

// gets the value from the string. the key can either be surrounded by
// double or single quotes, or just be plain text with escaped characters,
// possibly with a line-ending comment
fn parse_value(input: &str) -> IResult<&str, String> {
    alt((
        quote_surrounded,
        take_until_ws,
    ))(input)
}

// parse a single line of the config. every line with a config option is of form
// key=value #possible line ending comment.
fn parse_line(input: &str) -> Result<(&str, String), nom::Err<nom::error::Error<&str>>> {
    let (_, (k, v)) = separated_pair(parse_key, tag("="), parse_value)(input)?;
    Ok((k, v))
}

// parses a single netctl profile file from a &str. &str is expected to
// contain the profile file in lines.
pub fn parse_netctl_profile(config: &str) -> Option<netctl::Profile> {
    let mut profile = netctl::Profile::new();

    for line in config.lines() {
        match parse_line(line) {
            Ok(("Connection", v)) => profile.connection = netctl::Connection::from_str(&*v).unwrap(),
            Ok(("Interface", v)) => profile.interface = v,
            Ok(("ESSID", v)) => profile.essid = v,
            Ok(("Description", v)) => profile.description = v,
            Ok(("Security", v)) => {
                if v == "wpa-configsection" {return None} else {continue};
            },
            Ok(_) => continue, // line contained an unrecognized key or value
            Err(_) => continue, // line contained no key
        }
    }

    Some(profile)
}  