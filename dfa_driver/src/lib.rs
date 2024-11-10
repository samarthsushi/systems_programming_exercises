use std::collections::HashMap;

#[derive(Debug)]
pub struct DFAInner {
    num_states: usize,
    num_symbols: usize,
    delta: Vec<Vec<usize>>,
    final_states: Vec<usize>
}

impl DFAInner {
    pub fn new() -> Self {
        Self {
            num_states: 0,
            num_symbols: 0,
            delta: Vec::new(),
            final_states: Vec::new()
        }
    }
    
    pub fn build(&mut self, num_states: usize, num_symbols: usize) {
        self.num_states = num_states;
        self.num_symbols = num_symbols;
        self.delta = vec![vec![0; num_symbols]; num_states];
    }

    fn set_transition(&mut self, from_state: usize, symbol: usize, to_state: usize) {
        self.delta[from_state][symbol] = to_state;
    }

    fn add_final_state(&mut self, state: usize) {
        if !self.final_states.contains(&state) {
            self.final_states.push(state);
        }
    }

    fn is_final_state(&self, state: usize) -> bool {
        self.final_states.contains(&state)
    }
}

#[derive(Debug)]
pub struct DFA {
    dfa_inner: DFAInner,
    current_state: usize
}

impl DFA {
    pub fn new(dfa_inner: DFAInner, start_state: usize) -> Self {
        Self {
            dfa_inner,
            current_state: start_state
        }
    }

    pub fn validate(&mut self, input: &[usize]) -> bool {
        self.current_state = 0; 

        for &symbol in input {
            if symbol >= self.dfa_inner.num_symbols {
                eprintln!("ERR: symbol {} is not valid for this DFA", symbol);
                return false;
            }
            self.current_state = self.dfa_inner.delta[self.current_state][symbol];
        }

        self.dfa_inner.is_final_state(self.current_state)
    }

    pub fn from_string(dfa_input: &str) -> Self {
        let mut num_states = 0;
        let mut num_symbols = 0;
        let mut start_state = 0;
        let mut delta: Vec<Vec<usize>> = Vec::new();
        let mut final_states = Vec::new();
        let mut parsing_transitions = false;
        let mut obtained_states = false;
        let mut obtained_symbols = false;
        let mut dfa_inner = DFAInner::new();
        let mut dfa_unbuilt = true;

        for line in dfa_input.lines() {
            if line.is_empty() {
                continue;
            }   

            let trimmed_line = line.trim();

            if trimmed_line.starts_with("states:") {
                num_states = trimmed_line["states:".len()..]
                    .trim()
                    .parse()
                    .expect("Invalid number of states");
                obtained_states = true;
            } else if trimmed_line.starts_with("symbols:") {
                num_symbols = trimmed_line["symbols:".len()..]
                    .trim()
                    .parse()
                    .expect("Invalid number of symbols");
                obtained_symbols = true;
            } else if trimmed_line.starts_with("start_state:") {
                start_state = trimmed_line["start_state:".len()..]
                    .trim()
                    .parse()
                    .expect("Invalid start state");
            } else if trimmed_line.starts_with("final_states:") {
                final_states = trimmed_line["final_states:".len()..]
                    .split_whitespace()
                    .map(|s| s.trim().parse().expect("Invalid final state"))
                    .collect();
            } else if trimmed_line.starts_with("transitions:") {
                if !dfa_unbuilt {
                    parsing_transitions = true;
                }
            } else if parsing_transitions {
                let parts: Vec<_> = trimmed_line.split_whitespace().collect();
                if parts.len() == 3 {
                    let from_state: usize = parts[0].parse().expect("Invalid from state");
                    let symbol: usize = parts[1].parse().expect("Invalid symbol");
                    let to_state: usize = parts[2].parse().expect("Invalid to state");
                    dfa_inner.set_transition(from_state, symbol, to_state);
                }
            }

            if obtained_states && obtained_symbols && dfa_unbuilt {
                dfa_inner.build(num_states, num_symbols);
                dfa_unbuilt = false;
            }
        }

        if !parsing_transitions {
            eprintln!("ERR: preface delta with num symbols and states so it can be initialized");
        }
        dfa_inner.final_states = final_states;

        Self::new(dfa_inner, start_state)
    }
}

