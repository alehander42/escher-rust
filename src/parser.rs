use sexp::*;
use std::collections::HashMap;
use std::ascii::StrAsciiExt;

pub fn parse(source: &str) -> Result<Sexp, &'static str> {
    let arg = format!("({:s})", source);
    let cells = parse_cells(arg.as_slice());
    match cells {
        Ok(a) => {
            let result = Sexp{sexp: a.value, signatures: a.signatures};
            return Ok(result);
        }
        Err(message) => {
            return Err(message);
        }
    }
}

struct ParseResult {
    value:      Cell,
    left:       String,
    signatures: HashMap<String, TypeSignature>
}

struct SignatureParseResult {
    types: TypeSignature,
    left:        String
}

fn parse_cells(source: &str) -> Result<ParseResult, &'static str> {
    let trimmed_source = source.trim();

    let first = trimmed_source.char_at(0);
    if first == '{' {
        let l = parse_signature(trimmed_source.slice_from(1));
        match l {
            Ok(m) => {
                // hack to map signatures to fun and action forms
                let next = m.left.as_slice().trim();
                if !next.is_empty() && next.char_at(0) == '(' {
                    let mut z = parse_list(next.slice_from(1));
                    match z {
                        Ok(result) => {
                            match result.value {
                                ListCell(a) => {
                                    match *a.get(0) {
                                        StringCell(b) => {
                                            if b.as_slice() == "fun" || b.as_slice() == "action" {
                                                result.signatures.insert(b, m.types);
                                                return Ok(result);
                                            }
                                            else {
                                               return Err("type signature is valid only for fun");
                                            }
                                        }
                                        _ => {
                                           return Err("type signature is valid only for fun");
                                        }
                                    }
                                }
                                _ => {
                                    return Err("type signature is valid only for fun");
                                }
                            }
                        }
                        Err(message) => {
                            return Err(message);
                        }
                    }
                }
                else {
                    return Err("type signature is valid only for fun");
                }
            }
            Err(ex) => {
                return Err(ex);
            }
        }
    }
    else if first == '(' {
        return parse_list(trimmed_source.slice_from(1));
    }
    else if first == '"' {
        return Ok(parse_string(trimmed_source.slice_from(1)));
    }
    else if first.is_digit() {
        return Ok(parse_int(trimmed_source));
    }
    else { //if first.is_digit_radix(36) {
        return Ok(parse_ident(trimmed_source));
    }
}

fn parse_list(source: &str) -> Result<ParseResult, &'static str> {
    // parse 2 4 ) 2 -> ListCell([IntCell(2), IntCell(4)]), " 2"
    let mut list = vec![];
    let trimmed_source = source.trim();
    let mut first = trimmed_source.char_at(0);
    let mut left = String::from_str(trimmed_source);

    while first != ')' {
        let r = parse_cells(left.as_slice());
        match r {
            Ok(result) => {
                let l = result.left;
                left = l.to_string();
                list.push(result.value);
            }
            Err(message) => {
                return Err(message);
            }
        }

        if left.is_empty() {
            return Ok(ParseResult{value: ListCell(list), left: left, signatures: HashMap::new()});
        }
        else {
            first = left.as_slice().char_at(0);
        }
    }

    return Ok(ParseResult{value: ListCell(list), left: left, signatures: HashMap::new()});
}

fn parse_int(source: &str) -> ParseResult {
    // parse 23 ew -> IntCell(23), " ew"
    let mut number = 0u;
    let mut index = 0u;
    while index < source.len() && source.char_at(index).is_digit() {
        let digit = source.char_at(index).to_digit(10);
        match digit {
            Some(x) => {
                number = 10 * number + x;
                index += 1;
            }
            None => { break }
        }
    }
    return ParseResult{value: IntCell(number), left: source.slice_from(index).to_string(), signatures: HashMap::new()}
}

fn parse_ident(source: &str) -> ParseResult {
    let mut label = String::from_str("");
    let mut index = 0u;

    while index < source.len() && source.char_at(index) != ' ' {
        label.push_char(source.char_at(index));
        index += 1;
    }
    return ParseResult{value: IdentCell(label), left: source.slice_from(index).to_string(), signatures: HashMap::new()}
}

fn parse_string(source: &str) -> ParseResult {
    // parse e" 2 -> StringCell("e"), " 2"
    let mut label = String::from_str("");
    let mut index = 0u;

    while index < source.len() && source.char_at(index) != '"' {
        label.push_char(source.char_at(index));
        index += 1;
    }
    return ParseResult{value: StringCell(label), left: source.slice_from(index + 1).to_string(), signatures: HashMap::new()}
}

fn parse_signature(source: &str) -> Result<SignatureParseResult, &'static str> {
    let mut t = vec![];
    let mut parameters = vec![];
    let trimmed_source = source.trim();
    let mut first = trimmed_source.char_at(0);
    let mut left = String::from_str(trimmed_source);

    while first != '}' {
        let r = parse_cells(left.as_slice());
        match r {
            Ok(result) => {
                match result.value {
                    IdentCell(label) => {
                        if label.as_slice().char_at(0).is_uppercase() {
                            t.push(Type{label: label});
                        }
                        else if !parameters.contains(&label) {
                            parameters.push(ParameterType{label: label});
                        }
                        left = result.left;
                    }
                    _ => {
                        return Err("expected a label");
                    }
                }
            }
            Err(message) => { return Err(message); }
        }
        if left.is_empty() {
            return Ok(SignatureParseResult{types: TypeSignature{types: t, parameters: parameters}, left: left});
        }
        else {
            first = left.as_slice().char_at(0);
        }
    }

    return Ok(SignatureParseResult{types: TypeSignature{types: t, parameters: parameters}, left: left});
}
