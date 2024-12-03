use std::fs;
use std::error::Error;

enum State {
    SearchingInstruction,
    M,
    U,
    L,
    StartOperand1,
    Operand1,
    DoneOperand1,
    Operand2,
    DoneOperand2,
}

fn part1(input: String) -> u64 {

    let mut state = State::SearchingInstruction;
    let mut temp_operand = String::with_capacity(3);
    let mut operand1: u64 = 0;
    let mut operand2: u64 = 0;
    let mut result: u64 = 0;

    for char in input.chars() {
        match state {
            State::SearchingInstruction if char == 'm' => state = State::M,
            State::M if char == 'u' => state = State::U,
            State::U if char == 'l' => state = State::L,
            State::L if char == '(' => state = State::StartOperand1,
            State::StartOperand1 if char.is_ascii_digit() => { state = State::Operand1; temp_operand = String::with_capacity(3); temp_operand.push(char); },
            State::Operand1 if char.is_ascii_digit() => temp_operand.push(char),
            State::Operand1 if char == ',' => { state = State::DoneOperand1; operand1 = temp_operand.parse().unwrap(); },
            State::DoneOperand1 if char.is_ascii_digit() => { state = State::Operand2; temp_operand = String::with_capacity(3); temp_operand.push(char); },
            State::Operand2 if char.is_ascii_digit() => temp_operand.push(char),
            State::Operand2 if char == ')' => { state = State::SearchingInstruction; operand2 = temp_operand.parse().unwrap(); result = result + (operand1.saturating_mul(operand2)); },
            _ => state = State::SearchingInstruction
        }
    }

    result
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("part1: {}", part1(input));
}

