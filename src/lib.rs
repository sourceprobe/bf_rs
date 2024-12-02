use std::collections::HashMap;
use std::io::Write;

const VALID_CHARS : &str = "+-<>[]";

// Given a bf program, return a cleaned program
// which contains only valid instructions.
pub fn cleanup(program: &str) -> String {
    let mut output = String::new();
    for c in program.chars() {
        if VALID_CHARS.contains(c) {
            output.push(c);
        }
    }
    output
}

// Jumps maps the position of a brace to its
// match.
struct Jumps {
    // map from left t
    forward: HashMap<usize, usize>,
    backward: HashMap<usize, usize>,
}

impl Jumps {
    pub fn new() -> Jumps {
        Jumps {
            forward: HashMap::new(),
            backward: HashMap::new(),
        }
    }
    // find the matching right brace, or fail if none.
    pub fn right(&self, a: usize) -> Result<usize, ()> {
        match self.forward.get(&a) {
            Some(v) => Ok(*v),
            None => Err(()),
        }
    }

    // Find the matching left brace, or fail if none.
    pub fn left(&self, a: usize) -> Result<usize, ()> {
        match self.backward.get(&a) {
            Some(v) => Ok(*v),
            None => Err(()),
        }
    }
}

// Precalculates the position of matching braces.
// forward[i] is the position of the closing brace matching the opening brace
// at index i, and vice versa.
fn calculate_jumps(program: &str) -> Jumps {
    // stack of seen and unmatched open braces
    let mut open_positions = Vec::new();
    let mut jumps = Jumps::new();
    for (i,c) in program.chars().enumerate() {
        match c {
            '[' => {
                open_positions.push(i);
            },
            ']' => {
                let open = open_positions.pop().expect("Unmatched closing brace");
                jumps.forward.insert(open, i);
                jumps.backward.insert(i, open);
            },
            _ => {/*no op*/},
        }
    }
    assert!(open_positions.is_empty(), "Not all braces were matched.");

    jumps
}

// Execute the given program
pub fn run(program: &str) {
    let jumps = calculate_jumps(&program);
    let program = program.chars().collect::<Vec<char>>();
    let mut tape :[u8; 30000] = [0; 30000];
    let mut pc = 0;
    let mut ptr = 0;
    while pc < program.len() 
        && ptr >= 0 && 
        ptr < tape.len() {
        println!("pc = {pc}");
        match program[pc] {
            '+' => { tape[ptr] = tape[ptr].wrapping_add(1); },
            '-' => { tape[ptr] = tape[ptr].wrapping_sub(1); },
            '<' => { ptr = ptr.wrapping_sub(1); },
            '>' => { ptr = ptr.wrapping_add(1); },
            '.' => {
                std::io::stdout().write(&tape[ptr..ptr+1]);
            },
            '[' => {
                if tape[ptr] == 0 {
                    pc = jumps.right(pc).expect("bad jump");
                }
            },
            ']' => {
                if tape[ptr] != 0 {
                    pc = jumps.left(pc).expect("bad jump");
                }
            },

            x => {
                todo!("unimplemented instruction: '{x}'");
            }
        }
        pc += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(test)]
    fn test_clean_basic() {
        let input = "+- [[--<>]] comment";
        let expected ="+-[[--<>]] comment";
        assert_eq!(
            expected,
            cleanup(input));
    }

}
