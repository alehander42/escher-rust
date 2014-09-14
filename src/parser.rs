use sexp::*;

pub fn parse(source: &str) -> Cell {
    let arg = format!("({:s})", source);
    let cells = parse_cells(arg.as_slice());
    return cells.value;
}

struct ParseResult {
    value:   Cell,
    left:    String
}

fn parse_cells(source: &str) -> ParseResult {
    let trimmed_source = source.trim();

    let first = trimmed_source.char_at(0);
    if first == '(' {
        return parse_list(trimmed_source.slice_from(1));
    }
    else if first == '"' {
        return parse_string(trimmed_source.slice_from(1));
    }
    else if first.is_digit() {
        return parse_int(trimmed_source);
    }
    else { //if first.is_digit_radix(36) {
        return parse_ident(trimmed_source);
    }
}

fn parse_list(source: &str) -> ParseResult {
    // parse 2 4 ) 2 -> ListCell([IntCell(2), IntCell(4)]), " 2"
    let mut list = vec![];
    let trimmed_source = source.trim();
    let mut first = trimmed_source.char_at(0);
    let mut left = String::from_str(trimmed_source);

    while first != ')' {
        let result = parse_cells(left.as_slice());
        let l = result.left;
        left = l.to_string();
        list.push(result.value);

        if left.is_empty() {
            return ParseResult{value: ListCell(list), left: source.slice_from(source.len()).to_string()};
        }
        else {
            first = left.as_slice().char_at(0);
        }
    }

    return ParseResult{value: ListCell(list), left: left}
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
    return ParseResult{value: IntCell(number), left: source.slice_from(index).to_string()}
}

fn parse_ident(source: &str) -> ParseResult {
    // parse la 2 -> IdentCell("la"), " 2"
    let mut label = String::from_str("");
    let mut index = 0u;

    while index < source.len() && source.char_at(index) != ' ' {
        label.push_char(source.char_at(index));
        index += 1;
    }
    return ParseResult{value: IdentCell(label), left: source.slice_from(index).to_string()}
}

fn parse_string(source: &str) -> ParseResult {
    // parse e" 2 -> StringCell("e"), " 2"
    let mut label = String::from_str("");
    let mut index = 0u;

    while index < source.len() && source.char_at(index) != '"' {
        label.push_char(source.char_at(index));
        index += 1;
    }
    return ParseResult{value: StringCell(label), left: source.slice_from(index + 1).to_string()}
}
