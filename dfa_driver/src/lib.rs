struct DFA {
    num_states: usize,
    num_symbols: usize,
    delta: Vec<Vec<usize>>,
    final_states: Vec<usize>
}

impl DFA {
    fn new(num_states: usize, num_symbols: usize) -> Self {
        let delta = vec![vec![0; num_symbols]; num_states];

        Self {
            num_states,
            num_symbols,
            delta,
            final_states: Vec::new(), 
        }
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

struct DFADriver {
    dfa: DFA,
    current_state: usize
}

impl DFADriver {
    fn new(dfa: DFA, start_state: usize) -> Self {
        Self {
            dfa,
            current_state: start_state
        }
    }

    fn validate(&mut self, input: &[usize]) -> bool {
        self.current_state = 0; 

        for &symbol in input {
            if symbol >= self.dfa.num_symbols {
                eprintln!("ERR: symbol {} is not valid for this DFA", symbol);
                return false;
            }
            self.current_state = self.dfa.delta[self.current_state][symbol];
        }

        self.dfa.is_final_state(self.current_state)
    }
}