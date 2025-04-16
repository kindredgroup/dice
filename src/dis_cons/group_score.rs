use crate::dis_cons::group_score::Goals::{AtLeast, Exact};
use crate::logic::Push;
use crate::logic::con::Conjunction;
use crate::logic::dis::Disjunction;
use crate::{con, dis};
use std::cmp::max;
use std::fmt::{Display, Formatter};
use crate::dis_cons::DisCons;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Goals {
    Exact(usize),
    AtLeast(usize),
}

impl Display for Goals {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Goals::Exact(goals) => write!(f, "{goals}"),
            Goals::AtLeast(goals) => write!(f, "{goals}+"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player(pub usize);

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "p{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outcome(pub Player, pub Goals);

impl Display for Outcome {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} scores {}", self.0, self.1)
    }
}

fn for_players(num_players: usize, min_target_goals: usize, goal_cap: usize) -> DisCons<Outcome> {
    println!(
        "expanding {num_players} players, min_target_goals: {min_target_goals}, goal_cap: {goal_cap}"
    );
    if num_players >= 2 {
        let mut dis_cons = DisCons::default();
        for goals in 0..goal_cap {
            let outcome = Outcome(Player(num_players - 1), Exact(goals));
            let child_dis_cons = for_players(num_players - 1, min_target_goals.saturating_sub(1), goal_cap);
            println!("{outcome} with {child_dis_cons}");
            //TODO expand current outcome with child_discons
        }
        let outcome = Outcome(Player(num_players - 1), AtLeast(goal_cap));
        println!("... or {outcome}");
        dis_cons.push(con![outcome]);
        dis_cons
    } else {
        for_player(&Player(0), min_target_goals, goal_cap)
    }
}

fn for_player(
    player: &Player,
    min_target_goals: usize,
    goal_cap: usize,
) -> DisCons<Outcome> {
    let mut dis_cons = dis![];
    for goals in min_target_goals..goal_cap {
        dis_cons.push(con![Outcome(player.clone(), Exact(goals))])
    }
    dis_cons.push(con![Outcome(player.clone(), AtLeast(goal_cap))]);
    dis_cons
}

#[cfg(test)]
mod tests {
    use crate::{con, dis};
    use crate::dis_cons::group_score::Goals::{AtLeast, Exact};
    use crate::dis_cons::group_score::{Outcome, Player, for_player};

    #[test]
    fn transform_player() {
        let dis_cons = for_player(&Player(0), 2, 4);
        assert_eq!(
            dis![
                con![Outcome(Player(0), Exact(2))],
                con![Outcome(Player(0), Exact(3))],
                con![Outcome(Player(0), AtLeast(4))]
            ],
            dis_cons
        );
    }
}
