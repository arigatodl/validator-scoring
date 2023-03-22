/// The `Validator` struct represents a validator on the Ethereum network.
///
/// It stores data related to block signing, block proposals, attestations, slashings, and validator balance.
struct Validator {
    total_blocks_signed: u64, // The total number of blocks signed by the validator
    total_blocks_assigned: u64, // The total number of blocks assigned to the validator
    total_blocks_proposed: u64, // The total number of blocks proposed by the validator
    total_blocks_proposed_included: u64, // The total number of proposed blocks included in the Ethereum blockchain
    total_attestations_created: u64, // The total number of attestations created by the validator
    total_attestations_included: u64, // The total number of attestations included in the Ethereum blockchain
    number_of_slashings: u64,         // The total number of slashings the validator has undergone
    initial_validator_balance: f64, // The validator's initial balance when they joined the network
    current_validator_balance: f64, // The validator's current balance
}

/// The `Validator` struct represents a validator on the Ethereum network.
impl Validator {
    /// Calculates the uptime score (US) of a validator as a percentage.
    ///
    /// US = (Total blocks signed by validator / Total blocks assigned to validator) * 100
    fn uptime_score(&self) -> f64 {
        self.total_blocks_signed as f64 / self.total_blocks_assigned as f64 * 100.0
    }

    /// Calculates the proposal inclusion rate (PIR) of a validator as a percentage.
    ///
    /// PIR = (Total blocks proposed and included / Total blocks proposed) * 100
    fn proposal_inclusion_rate(&self) -> f64 {
        self.total_blocks_proposed_included as f64 / self.total_blocks_proposed as f64 * 100.0
    }

    /// Calculates the attestation inclusion rate (AIR) of a validator as a percentage.
    ///
    /// AIR = (Total attestations included / Total attestations created) * 100
    fn attestation_inclusion_rate(&self) -> f64 {
        self.total_attestations_included as f64 / self.total_attestations_created as f64 * 100.0
    }

    /// Calculates the slashing prevention (SP) factor for a validator.
    ///
    /// SP = 1 - (Number of slashings / Total number of validators)
    fn slashing_prevention(&self, total_validators: u64) -> f64 {
        1.0 - (self.number_of_slashings as f64 / total_validators as f64)
    }

    /// Calculates the balance growth (BG) of a validator as a ratio.
    ///
    /// BG = (Current validator balance - Initial validator balance) / Initial validator balance
    fn balance_growth(&self) -> f64 {
        (self.current_validator_balance - self.initial_validator_balance)
            / self.initial_validator_balance
    }

    /// Calculates the Ethereum validator score (EVS) for a validator, taking into account
    /// the weights for each factor:
    /// - Uptime Score (US): 40%
    /// - Proposal Inclusion Rate (PIR): 20%
    /// - Attestation Inclusion Rate (AIR): 20%
    /// - Slashing Prevention (SP): 10%
    /// - Balance Growth (BG): 10%
    ///
    /// EVS = (US * 0.4) + (PIR * 0.2) + (AIR * 0.2) + (SP * 0.1) + (BG * 0.1)
    fn ethereum_validator_score(&self, total_validators: u64) -> f64 {
        let us = self.uptime_score() * 0.40;
        let pir = self.proposal_inclusion_rate() * 0.20;
        let air = self.attestation_inclusion_rate() * 0.20;
        let sp = self.slashing_prevention(total_validators) * 0.10;
        let bg = self.balance_growth() * 0.10;

        us + pir + air + sp + bg
    }
}

fn main() {
    let validator = Validator {
        total_blocks_signed: 950,
        total_blocks_assigned: 1000,
        total_blocks_proposed: 100,
        total_blocks_proposed_included: 98,
        total_attestations_created: 1200,
        total_attestations_included: 1180,
        number_of_slashings: 0,
        initial_validator_balance: 32.0,
        current_validator_balance: 34.0,
    };

    let total_validators = 1000;
    let evs = validator.ethereum_validator_score(total_validators);
    println!("Ethereum Validator Score: {:.2}", evs);
}
