
use crate::lib::pda;
use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum PDAError {
    #[snafu(display("No accept states"))]
    NoStates,
    #[snafu(display("No accept states found"))]
    NoAccept
}


pub fn ensure_accept_nostates(pda: &pda::PDA) -> Result<&pda::PDA, PDAError> {
    if pda.accept_states.is_empty() {
        return NoAccept.fail()
    }
    if pda.states.is_empty() {
        return NoStates.fail()
    }
    Ok(pda)
}

/**
  * To ensure a single accept state, we set our accept state to a new state q_accept.
  * We also ensure that all previous accepting states have an epsilion transition to q_accept.
  */
pub fn single_accept(pda: &mut pda::PDA) -> Result<&pda::PDA, PDAError> {

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
                to_push.push(pda::Trans::new(format!("q_acc_{}", state), pda::EPSILON.into(), "".into(), "q_accept".into(), "".into()));
            }
        }
    }
    pda.transitions.append(&mut to_push);

    // Clear accept states and make our new accept state the only one
    pda.accept_states.clear();
    pda.accept_states.push("q_accept".into());

    Ok(pda)
}

pub fn empty_stack(pda: pda::PDA) -> pda::PDA {
    pda
}

pub fn pp_rule(pda: pda::PDA) -> pda::PDA {
    pda
}

pub fn pair_rule(pda: pda::PDA) -> pda::PDA {
    pda
}

pub fn eps_rule(pda: pda::PDA) -> pda::PDA {
    pda
}

pub fn ijk_rule(pda: pda::PDA) -> pda::PDA {
    pda
}
