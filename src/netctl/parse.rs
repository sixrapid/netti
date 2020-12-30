use std::{collections::HashMap};

use nom::{
    IResult, 
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::char,
    sequence::{delimited, separated_pair}
};

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


// get all key-value pairs in the config file and return in a hashmap
pub fn parse_profile_to_hashmap(config: &str) -> HashMap<&str, String> {
    let mut map = HashMap::new();

    for line in config.lines() {
        if let Ok((k, v)) = parse_line(line) {
            map.insert(k, v);
        }
    }

    map
}  