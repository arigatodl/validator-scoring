struct Validator {
    total_blocks_signed: u64,
    total_blocks_assigned: u64,
    total_blocks_proposed: u64,
    total_blocks_proposed_included: u64,
    total_attestations_created: u64,
    total_attestations_included: u64,
    number_of_slashings: u64,
    initial_validator_balance: f64,
    current_validator_balance: f64,
}

impl Validator {
    fn uptime_score(&self) -> f64 {
        self.total_blocks_signed as f64 / self.total_blocks_assigned as f64 * 100.0
    }

    fn proposal_inclusion_rate(&self) -> f64 {
        self.total_blocks_proposed_included as f64 / self.total_blocks_proposed as f64 * 100.0
    }

    fn attestation_inclusion_rate(&self) -> f64 {
        self.total_attestations_included as f64 / self.total_attestations_created as f64 * 100.0
    }

    fn slashing_prevention(&self, total_validators: u64) -> f64 {
        1.0 - (self.number_of_slashings as f64 / total_validators as f64)
    }

    fn balance_growth(&self) -> f64 {
        (self.current_validator_balance - self.initial_validator_balance)
            / self.initial_validator_balance
    }

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
