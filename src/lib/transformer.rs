use snafu::Snafu;

use crate::lib::cfg;
use crate::lib::pda;

use rand::prelude::*;

#[derive(Debug, Snafu)]
pub enum PDAError {
    #[snafu(display("No states"))]
    NoStates,
    #[snafu(display("No accept states found"))]
    NoAccept,
}

/// Ensures that the pda has states and accept states
pub fn ensure_accept_nostates(pda: pda::PDA) -> Result<pda::PDA, PDAError> {
    // TODO: Errors don't work like I want, due to the parser failing first (somewhat patched)
    if pda.accept_states.is_empty() {
        return NoAccept.fail();
    }
    if pda.states.is_empty() {
        return NoStates.fail();
    }
    Ok(pda)
}

/// Ensures the stack is emptied by pushing a hash at the beginning and making sure we remove it at the end
pub fn empty_stack(pda: &mut pda::PDA) -> Result<&pda::PDA, PDAError> {
    // Create our new starting state that pushes a hash
    pda.states.push(pda::START.into());
    pda.transitions.push(pda::Trans::new(
        pda::START.into(),
        pda::EPSILON.into(),
        pda::EPSILON.into(),
        pda.start_state.clone(),
        pda::HASH.into(),
    ));
    pda.start_state = pda::START.into();

    // Put tau transitions on each accepting state
    for acc_state in pda.accept_states.iter() {
        pda.transitions.push(pda::Trans::new(
            acc_state.clone(),
            pda::EPSILON.into(),
            pda::TAU.into(),
            acc_state.clone(),
            pda::EPSILON.into(),
        ))
    }
    Ok(pda)
}

/**
 * To ensure a single accept state, we set our accept state to a new state qF.
 * We also ensure that all previous accepting states have an epsilion transition to qF.
 */
pub fn single_accept(pda: &mut pda::PDA) -> () {
    pda.states.push(pda::ACCEPT.into());

    // If epsilon is not present, push it
    if !pda.input_alphabet.contains(&pda::EPSILON.into()) {
        pda.input_alphabet.push(pda::EPSILON.into());
    }

    // For every state, check if it is an accepting state, if so
    // Add a new eps transition from that state to the accept state.
    let pda_states = pda.states.iter_mut();
    let mut to_push: Vec<pda::Trans> = vec![];

    for state in pda_states {
        for final_state in pda.accept_states.iter() {
            if state.to_string().eq(final_state) {
                to_push.push(pda::Trans::new(
                    format!("A_acc-{}", state),
                    pda::EPSILON.into(),
                    pda::HASH.into(),
                    pda::ACCEPT.into(),
                    pda::EPSILON.into(),
                ));
            }
        }
    }
    pda.transitions.append(&mut to_push);

    // Clear accept states and make our new accept state the only one
    pda.accept_states.clear();
    pda.accept_states.push(pda::START.into());
}

/**
 * We need transitions to only push or pop a symbol. Not both, nor neither.
 */
pub fn only_push_pop(pda: &mut pda::PDA) -> () {
    let mut to_append: Vec<pda::Trans> = vec![];
    for trans in pda.transitions.iter_mut() {
        // If neither or both push/pop
        if trans.push.eq(pda::EPSILON.into()) && trans.pop.eq(pda::EPSILON.into())
            || !trans.push.eq(pda::EPSILON.into()) && !trans.pop.eq(pda::EPSILON.into())
        {
            let random: u16 = random();
            let new_state = format!("{}_P{}P", trans.state, random);
            pda.states.push(new_state.clone());

            let new_trans = pda::Trans::new(
                new_state.clone(),
                pda::EPSILON.into(),
                pda::SYMBOL.into(),
                trans.next.clone(),
                pda::EPSILON.into(),
            );
            trans.next = new_state;
            trans.push = pda::SYMBOL.into();
            to_append.push(new_trans);
        }
    }
    pda.transitions.append(&mut to_append);
}

/// For every state, make an epsilon rule
pub fn eps_rule(pda: &pda::PDA, cfg: &mut cfg::CFG) -> () {
    for state in pda
        .states
        .iter()
        .filter(|s| !(**s).eq(&pda::START.to_string()))
    {
        cfg.rules.push(cfg::Grammar::new(
            format!("A{}{}", state, state),
            pda::EPSILON.into(),
        ));
    }
}

/// For every triplet of states, Aij -> AikAkj
pub fn ijk_rule(pda: &pda::PDA, cfg: &mut cfg::CFG) -> () {
    for state_i in pda.states.iter()
    // Filter out our created accept state
    {
        for state_j in pda.states.iter() {
            'kloop: for state_k in pda.states.iter() {
                let rule_name = format!("A{}{}", state_i, state_j);
                let rule_desc = format!("A{}{}A{}{}", state_i, state_k, state_k, state_j);

                for mut rule in cfg.rules.iter_mut() {
                    if rule.rule_name.eq(&rule_name) {
                        rule.rule_desc = format!("{} | {}", rule.rule_desc, rule_desc);
                        continue 'kloop;
                    }
                }

                cfg.rules.push(cfg::Grammar::new(rule_name, rule_desc))
            }
        }
    }
}

/// For every stack symbol that could be pushed then popped, record states in the middle
pub fn pair_rule(pda: &pda::PDA, cfg: &mut cfg::CFG) -> () {
    for trans_a in pda
        .transitions
        .iter()
        // Ignore blanks
        .filter(|i| !i.input.eq(&pda::EPSILON.to_string()))
    {
        'bloop: for trans_b in pda
            .transitions
            .iter()
            .filter(|i| !i.input.eq(&pda::EPSILON.to_string()))
        {
            if trans_a.push.eq(&trans_b.pop) {
                let rule_desc = format!(
                    "{}A{}{}{}",
                    trans_a.input, trans_a.next, trans_b.next, trans_b.input
                );
                let rule_name = format!("A{}{}", trans_a.state, trans_b.state);

                for mut rule in cfg.rules.iter_mut() {
                    if rule.rule_name.eq(&rule_name) {
                        rule.rule_desc = format!("{} | {}", rule.rule_desc, rule_desc);
                        continue 'bloop;
                    }
                }

                cfg.rules.push(cfg::Grammar::new(rule_name, rule_desc))
            }
        }
    }
}
