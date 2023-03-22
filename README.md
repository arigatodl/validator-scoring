# validator-scoring

Open and verifiable scoring system for validators on the Ethereum network.

## Prerequisites

1. Install [rust](https://www.rust-lang.org/tools/install)

## Scoring method

1. Uptime Score (US): The percentage of time a validator is online and actively participating in the network. A higher uptime indicates a more reliable validator. This can be calculated using the following formula:

US = (Total blocks signed by validator / Total blocks assigned to validator) * 100

1. Proposal Inclusion Rate (PIR): The percentage of the validator's proposed blocks that are included in the Ethereum blockchain. This measures the efficiency and effectiveness of a validator's block proposals. The formula is:

PIR = (Total blocks proposed and included / Total blocks proposed) * 100

1. Attestation Inclusion Rate (AIR): The percentage of a validator's attestations that are included in the Ethereum blockchain. Attestations are votes on the validity of proposed blocks. A high AIR indicates that the validator's attestations are valuable to the network. The formula is:

AIR = (Total attestations included / Total attestations created) * 100

1. Slashing Prevention (SP): Validators should not be slashed (penalized) for misbehavior. This factor penalizes validators who have been slashed. The formula is:

SP = 1 - (Number of slashings / Total number of validators)

1. Balance Growth (BG): The total rewards earned by a validator, taking into account both block proposals and attestations. This indicates the validator's overall profitability. The formula is:

BG = (Current validator balance - Initial validator balance) / Initial validator balance


Now, to create a composite score that takes into account all these factors, you can assign weights to each factor based on their importance. Here's a suggested weighting:

Uptime Score (US): 40%
Proposal Inclusion Rate (PIR): 20%
Attestation Inclusion Rate (AIR): 20%
Slashing Prevention (SP): 10%
Balance Growth (BG): 10%
The final Ethereum validator score (EVS) can be calculated as follows:

EVS = (US * 0.4) + (PIR * 0.2) + (AIR * 0.2) + (SP * 0.1) + (BG * 0.1)

This scoring system is fair and provable, as it considers a range of factors that contribute to a validator's performance on the Ethereum network. Additionally, it encourages validators to maintain high uptime, propose and attest efficiently, avoid slashing events, and contribute to the overall security and decentralization of the network.
