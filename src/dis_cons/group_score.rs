use crate::dis_cons::group_score::Goals::{AtLeast, Exact};
use crate::dis_cons::{DisCons, expand_dis_cons};
use crate::logic::{IntoInner, New, Push};
use crate::{con, dis};
use std::fmt::{Display, Formatter};
use crate::logic::dis::Disjunction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Goals {
    Exact(usize),
    AtLeast(usize),
}

impl Display for Goals {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Exact(goals) => write!(f, "{goals}"),
            AtLeast(goals) => write!(f, "{goals}+"),
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
        let mut dis_cons = DisCons::with_capacity(goal_cap + 1);
        for goals in 0..goal_cap {
            let outcome = Outcome(Player(num_players - 1), Exact(goals));
            let child_dis_cons = for_players(
                num_players - 1,
                min_target_goals.saturating_sub(goals),
                goal_cap,
            );
            println!("{outcome} with {child_dis_cons}");
            let expanded_dis_cons = expand_dis_cons(&outcome, &child_dis_cons);
            for disjunct in expanded_dis_cons.into_inner() {
                dis_cons.push(disjunct);
            }
        }
        let outcome = Outcome(Player(num_players - 1), AtLeast(goal_cap));
        println!("... or {outcome}");
        dis_cons.push(con![outcome]);
        dis_cons
    } else {
        for_player(&Player(0), min_target_goals, goal_cap)
    }
}

#[inline]
fn for_player(player: &Player, min_target_goals: usize, goal_cap: usize) -> DisCons<Outcome> {
    let mut dis_cons = Disjunction::with_capacity(goal_cap - min_target_goals + 1);
    for goals in min_target_goals..goal_cap {
        dis_cons.push(con![Outcome(player.clone(), Exact(goals))])
    }
    dis_cons.push(con![Outcome(player.clone(), AtLeast(goal_cap))]);
    dis_cons
}

#[cfg(test)]
mod tests {
    use crate::dis_cons::group_score::Goals::{AtLeast, Exact};
    use crate::dis_cons::group_score::{Outcome, Player, for_player, for_players};
    use crate::{con, dis};

    #[test]
    fn transform_players_2_3_3() {
        let dis_cons = for_players(2, 3, 3);
        println!("{dis_cons}");
        assert_eq!(
            dis![
                con![Outcome(Player(1), Exact(0)), Outcome(Player(0), AtLeast(3))],
                con![Outcome(Player(1), Exact(1)), Outcome(Player(0), Exact(2))],
                con![Outcome(Player(1), Exact(1)), Outcome(Player(0), AtLeast(3))],
                con![Outcome(Player(1), Exact(2)), Outcome(Player(0), Exact(1))],
                con![Outcome(Player(1), Exact(2)), Outcome(Player(0), Exact(2))],
                con![Outcome(Player(1), Exact(2)), Outcome(Player(0), AtLeast(3))],
                con![Outcome(Player(1), AtLeast(3))]
            ],
            dis_cons
        );
    }

    #[test]
    fn transform_player_2_4() {
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

    #[test]
    fn transform_player_3_4() {
        let dis_cons = for_player(&Player(0), 3, 4);
        assert_eq!(
            dis![
                con![Outcome(Player(0), Exact(3))],
                con![Outcome(Player(0), AtLeast(4))]
            ],
            dis_cons
        );
    }
}
