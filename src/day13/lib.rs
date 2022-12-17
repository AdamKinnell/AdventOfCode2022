
#[derive(PartialEq, Eq)]
pub enum Element<'a> {
    List(&'a str),
    Integer(i32)
}

#[derive(PartialEq, Eq, Debug)]
pub enum ComparisonResult {
    CorrectOrder,
    IncorrectOrder,
    Indeterminate,
}

pub fn parse_list(data: &str) -> Vec<Element> {
    let mut depth = 0;
    let mut chars = data.chars().enumerate().peekable();
    let mut elements = Vec::new();
    let mut i_nested_index_start = 0;
    let mut in_integer = false;
    loop {
        let (i, char) = chars.next().unwrap();

        if char == ',' {
            in_integer = false;
            continue;
        }

        if depth == 0 {
            // We haven't started parsing yet
            match char {
                '[' => depth += 1,
                _ => unreachable!()
            }
        } else if depth == 1 {
            // Looking at the  root list
            match char {
                '0'..='9' => {
                    let digit = char as i32 - '0' as i32;
                    if in_integer {
                        if let Some(Element::Integer(num)) = elements.last_mut() {
                            // Another digit of the same number
                            *num = (*num * 10) + digit;
                        }
                    } else {
                        // A new number
                        elements.push(Element::Integer(digit));
                        in_integer = true
                    }
                },
                '[' => {
                    i_nested_index_start = i;
                    depth += 1;
                },
                ']' => return elements,
                _ => unreachable!()
            }
        } else {
            // Part of nested list and we only care about getting out of it
            match char {
                '[' => depth += 1,
                ']' if depth == 2 => {
                    // End of directly nested list
                    elements.push(Element::List(&data[i_nested_index_start..=i]));
                    depth -= 1
                },
                ']' => depth -= 1,
                _ => continue
            }
        }
    }
}

pub fn compare_data(left: &str, right: &str) -> ComparisonResult {
    let left_list = parse_list(left);
    let right_list = parse_list(right);

    let zipped = left_list.iter().zip(right_list.iter());
    for (left, right) in zipped {
        let result;
        match (left, right) {
            (Element::List(left_nested), Element::List(right_nested)) => {
                // Compare two lists
                result = compare_data(left_nested, right_nested);
            },
            (Element::List(left_nested), Element::Integer(right_int)) => {
                // Compare an int and a list
                let mut right_list = String::with_capacity(4);
                right_list.push('[');
                right_list.push_str(&right_int.to_string());
                right_list.push(']');

                result = compare_data(left_nested, &right_list);
            },
            (Element::Integer(left_int), Element::List(right_nested)) => {
                // Compare an int and a list
                let mut left_list = String::with_capacity(4);
                left_list.push('[');
                left_list.push_str(&left_int.to_string());
                left_list.push(']');

                result = compare_data(&left_list, right_nested);
            },
            (Element::Integer(left_int), Element::Integer(right_int)) => {
                // Compare two integers
                if *left_int > *right_int {
                    result = ComparisonResult::IncorrectOrder;
                } else if *left_int < *right_int {
                    result = ComparisonResult::CorrectOrder;
                } else {
                    result = ComparisonResult::Indeterminate;
                }
            },
        }

        match result {
            ComparisonResult::CorrectOrder | ComparisonResult::IncorrectOrder => return result,
            ComparisonResult::Indeterminate => continue,
        }
    }

    // "If the left list runs out of items first, the inputs are in the right order."
    if left_list.len() < right_list.len() {
        return ComparisonResult::CorrectOrder;
    }

    // "If the right list runs out of items first, the inputs are not in the right order."
    if left_list.len() > right_list.len() {
        return ComparisonResult::IncorrectOrder;
    }

    return ComparisonResult::Indeterminate;
}