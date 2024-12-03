use std::fs;

enum State {
    SearchingInstruction,
    M,
    U,
    L,
    StartOperand1,
    Operand1,
    DoneOperand1,
    Operand2,
    StartConditional,
    InConditional,
    InDoConditional,
    InDontConditional,
    InDontConditionalAp,
    InDontConditionalT,
    InDontConditionalB1,
}

fn part(input: &str, honor_conditionals: bool) -> u64 {
    let mut state = State::SearchingInstruction;
    let mut mul_enabled = true;
    let mut temp_operand = String::with_capacity(3);
    let mut operand1: u64 = 0;
    let mut result: u64 = 0;

    for char in input.chars() {
        match state {
            State::StartConditional if char == 'o' => state = State::InConditional,
            State::InConditional if char == '(' => state = State::InDoConditional,
            State::InDoConditional if char == ')' => {
                state = State::SearchingInstruction;
                mul_enabled = true;
            }
            State::InConditional if char == 'n' => state = State::InDontConditional,
            State::InDontConditional if char == '\'' => state = State::InDontConditionalAp,
            State::InDontConditionalAp if char == 't' => state = State::InDontConditionalT,
            State::InDontConditionalT if char == '(' => state = State::InDontConditionalB1,
            State::InDontConditionalB1 if char == ')' => {
                state = State::SearchingInstruction;
                mul_enabled = false;
            }
            State::SearchingInstruction if char == 'm' => state = State::M,
            State::M if char == 'u' => state = State::U,
            State::U if char == 'l' => state = State::L,
            State::L if char == '(' => state = State::StartOperand1,
            State::StartOperand1 if char.is_ascii_digit() => {
                state = State::Operand1;
                temp_operand = String::with_capacity(3);
                temp_operand.push(char);
            }
            State::Operand1 if char.is_ascii_digit() => temp_operand.push(char),
            State::Operand1 if char == ',' => {
                state = State::DoneOperand1;
                operand1 = temp_operand.parse().unwrap();
            }
            State::DoneOperand1 if char.is_ascii_digit() => {
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
                state = {
                    if char == 'd' {
                        State::StartConditional
                    } else {
                        State::SearchingInstruction
                    }
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
