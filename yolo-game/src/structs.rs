use super::*;

#[cw_serde]
pub struct Stake {
    pub player: Addr,
    pub share: Uint128,
}

impl Stake {
    pub fn new(player: Addr, share: Uint128) -> Self {
        Self { player, share }
    }

    pub fn update_share(&mut self, share: Uint128) {
        self.share += share;
    }
}

#[cw_serde]
#[derive(Default)]
pub struct ScoreBoard {
    pub stakes: Vec<Stake>,
    pub status: Status,
}

impl ScoreBoard {
    pub fn update_status(&mut self, status: Status) {
        self.status = status;
    }

    pub fn is_player_staked(&self, player: &Addr) -> bool {
        for stake in self.stakes.iter() {
            if stake.player.eq(&player) {
                return true;
            }
        }
        false
    }

    pub fn add(&mut self, player: Addr, share: Uint128) {
        for stake in self.stakes.iter_mut() {
            if stake.player.eq(&player) {
                stake.share += share;
                break;
            }
        }

        if !self.stakes.iter().any(|stake| stake.player.eq(&player)) {
            self.stakes.push(Stake::new(player, share))
        }
    }

    pub fn convert_to_view(&self) -> ScoreBoardView {
        let mut biggest_stake = Stake::new(Addr::unchecked(""), Uint128::default());

        for stake in self.stakes.iter() {
            if stake.share > biggest_stake.share {
                biggest_stake = stake.clone();
            }
        }

        ScoreBoardView {
            scores: self.clone(),
            biggest_stake,
        }
    }
}

#[cw_serde]
pub struct ScoreBoardView {
    pub scores: ScoreBoard,
    pub biggest_stake: Stake,
}
