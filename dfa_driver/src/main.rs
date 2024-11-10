fn main() {
    let dfa_inner = dfa_driver::DFAInner::new();
    let dfa_description = r#"
        states: 3
        symbols: 2
        start_state: 1
        final_states: 2
        transitions:
            0 0 1
            0 1 2
            1 0 1
            1 1 2
            2 0 0
            2 1 0
    "#;

    let dfa = dfa_driver::DFA::from_string(dfa_description);

    println!("{:#?}", dfa);
}
