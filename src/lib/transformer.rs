
use crate::lib::pda;
use crate::lib::cfg;
use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum PDAError {
    #[snafu(display("No accept states"))]
    NoStates,
    #[snafu(display("No accept states found"))]
    NoAccept
}

/// Ensures that the pda has states and accept states
pub fn ensure_accept_nostates(pda: pda::PDA) -> Result<pda::PDA, PDAError> { // Errors don't work like I want, due to the parser failing first
    if pda.accept_states.is_empty() {
        return NoAccept.fail()
    }
    if pda.states.is_empty() {
        return NoStates.fail()
    }
    Ok(pda)
}

pub fn empty_stack(pda: &mut pda::PDA) -> Result<&pda::PDA, PDAError> {
    Ok(pda)
}

/**
  * To ensure a single accept state, we set our accept state to a new state q_accept.
  * We also ensure that all previous accepting states have an epsilion transition to q_accept.
  */
pub fn single_accept(pda: &mut pda::PDA) -> () {

    // If epsilon is not present, push it
    pda.states.push("q_accept".into());
    if !pda.input_alphabet.contains(&pda::EPSILON.into()) { // no double eps transition
        pda.input_alphabet.push(pda::EPSILON.into());
    }

    // For every state, check if it is an accepting state, if so
    // Add a new eps transition from that state to the accept state.
    let pda_states = pda.states.iter_mut();
    let mut to_push: Vec<pda::Trans> = vec![];

    for state in pda_states {
        for final_state in pda.accept_states.iter() {
            if state.to_string().eq(final_state) {
                to_push.push(pda::Trans::new(format!("A_acc-{}", state), pda::EPSILON.into(), "".into(), "q_accept".into(), "".into()));
            }
        }
    }
    pda.transitions.append(&mut to_push);

    // Clear accept states and make our new accept state the only one
    pda.accept_states.clear();
    pda.accept_states.push("q_accept".into());

}

/// For every state, make an epsilon rule
pub fn eps_rule(pda: &pda::PDA, cfg: &mut cfg::CFG) -> () {
    for state in pda.states.iter() {
        if state.eq("q_accept") { //Not necessary
            continue;
        }
        cfg.rules.push(cfg::Grammar::new(format!("A_{}_{}", state, state), pda::EPSILON.into()));
    }
}

pub fn ijk_rule(pda: &pda::PDA, cfg: &mut cfg::CFG) -> () {
    for state_i in pda.states.iter() {
        for state_j in pda.states.iter() {
            for state_k in pda.states.iter() {
                if state_i.eq("q_accept") || state_j.eq("q_accept") || state_k.eq("q_accept") {
                    continue;
                }
                let rule_desc = format!("A_{}{}A_{}{}", state_i, state_k, state_j, state_k);
                cfg.rules.push(cfg::Grammar::new(format!("A_{}{}", state_i, state_j), rule_desc));
            }
        }
    }
}

pub fn pair_rule(pda: &pda::PDA, cfg: &mut cfg::CFG) -> () {
    for trans_a in pda.transitions.iter() {
        for trans_b in pda.transitions.iter() {
            if trans_a.new.eq(&trans_b.input) {
                if trans_a.new.eq(&"~".to_string()) { // ignore blanks
                    continue;
                }
                let rule_desc = format!("{}A_{}{}{}", trans_a.new, trans_a.state, trans_b.state, trans_b.new);
                let g = cfg::Grammar::new(format!("A_{}_{}", trans_a.state, trans_b.state), rule_desc);
                cfg.rules.push(g)
            }
        }
    }
}
