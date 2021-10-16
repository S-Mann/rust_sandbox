use rand::seq::SliceRandom;
use std::collections::HashMap;

fn expander(sym: &str, expansion: &mut Vec<String>, grammar: &HashMap<&str, Vec<Vec<&str>>>) {
    if let Some(next_sym) = grammar.get(sym) {
        if let Some(exp_sym) = next_sym.choose(&mut rand::thread_rng()) {
            for trg_exp in exp_sym {
                expander(trg_exp, expansion, grammar);
            }
        }
    } else {
        expansion.push(sym.to_string());
    }
}

fn main() {
    let mut grammar = HashMap::new();
    grammar.insert(
        "S",
        vec![
            vec!["S", "op", "S"],
            vec!["num"],
        ],
    );
    grammar.insert(
        "num",
        vec![
            vec!["num", "num"],
            vec!["0"],
            vec!["1"],
            vec!["2"],
            vec!["3"],
            vec!["4"],
            vec!["6"],
            vec!["7"],
            vec!["8"],
            vec!["9"],
        ],
    );
    grammar.insert("op", vec![vec!["+"], vec!["+"], vec!["*"], vec!["/"]]);

    let mut expansion: Vec<String> = vec![];

    expander("S", &mut expansion, &grammar);
    println!("Generated expression -- {:?}", expansion.join(""));
}
