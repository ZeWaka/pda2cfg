
use crate::lib::pda;
use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum PDAError {
    #[snafu(display("No accept states"))]
    NoStates,
    #[snafu(display("No accept states found"))]
    NoAccept
}


pub fn ensure_accept(pda: &pda::PDA) -> Result<&pda::PDA, PDAError> {
    if pda.accept_states.is_empty() {
        return NoAccept.fail()
    }
    Ok(pda)
}

pub fn single_accept(pda: &pda::PDA) -> Result<&pda::PDA, PDAError> {
    // if pda.accept_states.is_empty() {
    //     return NoAccept.fail()
    // }
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
