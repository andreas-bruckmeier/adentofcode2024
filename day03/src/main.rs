use std::fs;

enum State {
    SearchingInstruction,
    MulM,
    MulU,
    MulL,
    StartOperand1,
    Operand1,
    StartOperand2,
    Operand2,
    StartConditional,
    ConditionalO,
    DoConditionalBracket,
    DontConditionalN,
    DontConditionalAP,
    DontConditionalT,
    DontConditionalBracket,
}

fn part(input: &str, honor_conditionals: bool) -> u64 {
    let mut state = State::SearchingInstruction;
    let mut mul_enabled = true;
    let mut temp_operand = String::with_capacity(3);
    let mut operand1: u64 = 0;
    let mut result: u64 = 0;

    for char in input.chars() {
        match state {
            State::StartConditional if char == 'o' => state = State::ConditionalO,
            State::ConditionalO if char == '(' => state = State::DoConditionalBracket,
            State::DoConditionalBracket if char == ')' => {
                state = State::SearchingInstruction;
                mul_enabled = true;
            }
            State::ConditionalO if char == 'n' => state = State::DontConditionalN,
            State::DontConditionalN if char == '\'' => state = State::DontConditionalAP,
            State::DontConditionalAP if char == 't' => state = State::DontConditionalT,
            State::DontConditionalT if char == '(' => state = State::DontConditionalBracket,
            State::DontConditionalBracket if char == ')' => {
                state = State::SearchingInstruction;
                mul_enabled = false;
            }
            State::SearchingInstruction if char == 'm' => state = State::MulM,
            State::MulM if char == 'u' => state = State::MulU,
            State::MulU if char == 'l' => state = State::MulL,
            State::MulL if char == '(' => state = State::StartOperand1,
            State::StartOperand1 if char.is_ascii_digit() => {
                state = State::Operand1;
                temp_operand = String::with_capacity(3);
                temp_operand.push(char);
            }
            State::Operand1 if char.is_ascii_digit() => temp_operand.push(char),
            State::Operand1 if char == ',' => {
                state = State::StartOperand2;
                operand1 = temp_operand.parse().unwrap();
            }
            State::StartOperand2 if char.is_ascii_digit() => {
                state = State::Operand2;
                temp_operand = String::with_capacity(3);
                temp_operand.push(char);
            }
            State::Operand2 if char.is_ascii_digit() => temp_operand.push(char),
            State::Operand2 if char == ')' => {
                state = State::SearchingInstruction;
                if !honor_conditionals || mul_enabled {
                    result += operand1.saturating_mul(temp_operand.parse().unwrap());
                }
            }
            _ => {
                state = match char {
                    'd' => State::StartConditional,
                    'm' => State::MulM,
                    _ => State::SearchingInstruction
                }
            }
        }
    }

    result
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("part1: {}", part(&input, false));
    println!("part2: {}", part(&input, true));
}
