use crate::dis_cons::group_score::Goals::{AtLeast, Exactly};
use crate::dis_cons::{expand_dis_cons, DisCons};
use crate::logic::dis::Disjunction;
use crate::logic::{IntoInner, New, Push};
use crate::con;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Goals {
    Exactly(usize),
    AtLeast(usize),
}

impl Display for Goals {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Exactly(goals) => write!(f, "{goals}"),
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

pub fn transform(players: usize, min_target_goals: usize, goal_cap: usize) -> DisCons<Outcome> {
    assert!(players > 0, "players ({players}) must be greater than 0");
    
    if players >= 2 {
        let mut dis_cons = DisCons::default();
        for goals in 0..goal_cap {
            let outcome = Outcome(Player(players - 1), Exactly(goals));
            let child_dis_cons = transform(
                players - 1,
                min_target_goals.saturating_sub(goals),
                goal_cap,
            );
            let expanded_dis_cons = expand_dis_cons(&outcome, &child_dis_cons);
            dis_cons.push_all(expanded_dis_cons.into_inner());
        }
        let outcome = Outcome(Player(players - 1), AtLeast(goal_cap));
        dis_cons.push(con![outcome]);
        dis_cons
    } else {
        for_player(&Player(0), min_target_goals, goal_cap)
    }
}

#[inline]
fn for_player(player: &Player, min_target_goals: usize, goal_cap: usize) -> DisCons<Outcome> {
    assert!(min_target_goals <= goal_cap, "min_target_goals ({min_target_goals}) cannot exceed goal_cap ({goal_cap})");
    
    let mut dis_cons = Disjunction::with_capacity(goal_cap - min_target_goals + 1);
    for goals in min_target_goals..goal_cap {
        dis_cons.push(con![Outcome(player.clone(), Exactly(goals))])
    }
    dis_cons.push(con![Outcome(player.clone(), AtLeast(goal_cap))]);
    dis_cons
}

#[cfg(test)]
mod tests {
    use crate::dis_cons::group_score::Goals::{AtLeast, Exactly};
    use crate::dis_cons::group_score::{for_player, transform, Outcome, Player};
    use crate::{con, dis};

    #[test]
    fn transform_3_players_2_3() {
        let dis_cons = transform(3, 2, 3);
        println!("{dis_cons}");
        assert_eq!(
            dis![
                con![Outcome(Player(2), Exactly(0)), Outcome(Player(1), Exactly(0)), Outcome(Player(0), Exactly(2))],
                con![Outcome(Player(2), Exactly(0)), Outcome(Player(1), Exactly(0)), Outcome(Player(0), AtLeast(3))],
                con![Outcome(Player(2), Exactly(0)), Outcome(Player(1), Exactly(1)), Outcome(Player(0), Exactly(1))],
                con![Outcome(Player(2), Exactly(0)), Outcome(Player(1), Exactly(1)), Outcome(Player(0), Exactly(2))],
                con![Outcome(Player(2), Exactly(0)), Outcome(Player(1), Exactly(1)), Outcome(Player(0), AtLeast(3))],
                con![Outcome(Player(2), Exactly(0)), Outcome(Player(1), Exactly(2)), Outcome(Player(0), Exactly(0))],
                con![Outcome(Player(2), Exactly(0)), Outcome(Player(1), Exactly(2)), Outcome(Player(0), Exactly(1))],
                con![Outcome(Player(2), Exactly(0)), Outcome(Player(1), Exactly(2)), Outcome(Player(0), Exactly(2))],
                con![Outcome(Player(2), Exactly(0)), Outcome(Player(1), Exactly(2)), Outcome(Player(0), AtLeast(3))],
                con![Outcome(Player(2), Exactly(0)), Outcome(Player(1), AtLeast(3))],
                // ---
                con![Outcome(Player(2), Exactly(1)), Outcome(Player(1), Exactly(0)), Outcome(Player(0), Exactly(1))],
                con![Outcome(Player(2), Exactly(1)), Outcome(Player(1), Exactly(0)), Outcome(Player(0), Exactly(2))],
                con![Outcome(Player(2), Exactly(1)), Outcome(Player(1), Exactly(0)), Outcome(Player(0), AtLeast(3))],
                con![Outcome(Player(2), Exactly(1)), Outcome(Player(1), Exactly(1)), Outcome(Player(0), Exactly(0))],
                con![Outcome(Player(2), Exactly(1)), Outcome(Player(1), Exactly(1)), Outcome(Player(0), Exactly(1))],
                con![Outcome(Player(2), Exactly(1)), Outcome(Player(1), Exactly(1)), Outcome(Player(0), Exactly(2))],
                con![Outcome(Player(2), Exactly(1)), Outcome(Player(1), Exactly(1)), Outcome(Player(0), AtLeast(3))],
                con![Outcome(Player(2), Exactly(1)), Outcome(Player(1), Exactly(2)), Outcome(Player(0), Exactly(0))],
                con![Outcome(Player(2), Exactly(1)), Outcome(Player(1), Exactly(2)), Outcome(Player(0), Exactly(1))],
                con![Outcome(Player(2), Exactly(1)), Outcome(Player(1), Exactly(2)), Outcome(Player(0), Exactly(2))],
                con![Outcome(Player(2), Exactly(1)), Outcome(Player(1), Exactly(2)), Outcome(Player(0), AtLeast(3))],
                con![Outcome(Player(2), Exactly(1)), Outcome(Player(1), AtLeast(3))],
                // ---
                con![Outcome(Player(2), Exactly(2)), Outcome(Player(1), Exactly(0)), Outcome(Player(0), Exactly(0))],
                con![Outcome(Player(2), Exactly(2)), Outcome(Player(1), Exactly(0)), Outcome(Player(0), Exactly(1))],
                con![Outcome(Player(2), Exactly(2)), Outcome(Player(1), Exactly(0)), Outcome(Player(0), Exactly(2))],
                con![Outcome(Player(2), Exactly(2)), Outcome(Player(1), Exactly(0)), Outcome(Player(0), AtLeast(3))],
                con![Outcome(Player(2), Exactly(2)), Outcome(Player(1), Exactly(1)), Outcome(Player(0), Exactly(0))],
                con![Outcome(Player(2), Exactly(2)), Outcome(Player(1), Exactly(1)), Outcome(Player(0), Exactly(1))],
                con![Outcome(Player(2), Exactly(2)), Outcome(Player(1), Exactly(1)), Outcome(Player(0), Exactly(2))],
                con![Outcome(Player(2), Exactly(2)), Outcome(Player(1), Exactly(1)), Outcome(Player(0), AtLeast(3))],
                con![Outcome(Player(2), Exactly(2)), Outcome(Player(1), Exactly(2)), Outcome(Player(0), Exactly(0))],
                con![Outcome(Player(2), Exactly(2)), Outcome(Player(1), Exactly(2)), Outcome(Player(0), Exactly(1))],
                con![Outcome(Player(2), Exactly(2)), Outcome(Player(1), Exactly(2)), Outcome(Player(0), Exactly(2))],
                con![Outcome(Player(2), Exactly(2)), Outcome(Player(1), Exactly(2)), Outcome(Player(0), AtLeast(3))],
                con![Outcome(Player(2), Exactly(2)), Outcome(Player(1), AtLeast(3))],
                // ---
                con![Outcome(Player(2), AtLeast(3))]
            ],
            dis_cons
        );
    }

    #[test]
    fn transform_3_players_3_3() {
        let dis_cons = transform(3, 3, 3);
        println!("{dis_cons}");
        assert_eq!(
            dis![
                con![Outcome(Player(2), Exactly(0)), Outcome(Player(1), Exactly(0)), Outcome(Player(0), AtLeast(3))],
                con![Outcome(Player(2), Exactly(0)), Outcome(Player(1), Exactly(1)), Outcome(Player(0), Exactly(2))],
                con![Outcome(Player(2), Exactly(0)), Outcome(Player(1), Exactly(1)), Outcome(Player(0), AtLeast(3))],
                con![Outcome(Player(2), Exactly(0)), Outcome(Player(1), Exactly(2)), Outcome(Player(0), Exactly(1))],
                con![Outcome(Player(2), Exactly(0)), Outcome(Player(1), Exactly(2)), Outcome(Player(0), Exactly(2))],
                con![Outcome(Player(2), Exactly(0)), Outcome(Player(1), Exactly(2)), Outcome(Player(0), AtLeast(3))],
                con![Outcome(Player(2), Exactly(0)), Outcome(Player(1), AtLeast(3))],
                // ---
                con![Outcome(Player(2), Exactly(1)), Outcome(Player(1), Exactly(0)), Outcome(Player(0), Exactly(2))],
                con![Outcome(Player(2), Exactly(1)), Outcome(Player(1), Exactly(0)), Outcome(Player(0), AtLeast(3))],
                con![Outcome(Player(2), Exactly(1)), Outcome(Player(1), Exactly(1)), Outcome(Player(0), Exactly(1))],
                con![Outcome(Player(2), Exactly(1)), Outcome(Player(1), Exactly(1)), Outcome(Player(0), Exactly(2))],
                con![Outcome(Player(2), Exactly(1)), Outcome(Player(1), Exactly(1)), Outcome(Player(0), AtLeast(3))],
                con![Outcome(Player(2), Exactly(1)), Outcome(Player(1), Exactly(2)), Outcome(Player(0), Exactly(0))],
                con![Outcome(Player(2), Exactly(1)), Outcome(Player(1), Exactly(2)), Outcome(Player(0), Exactly(1))],
                con![Outcome(Player(2), Exactly(1)), Outcome(Player(1), Exactly(2)), Outcome(Player(0), Exactly(2))],
                con![Outcome(Player(2), Exactly(1)), Outcome(Player(1), Exactly(2)), Outcome(Player(0), AtLeast(3))],
                con![Outcome(Player(2), Exactly(1)), Outcome(Player(1), AtLeast(3))],
                // ---
                con![Outcome(Player(2), Exactly(2)), Outcome(Player(1), Exactly(0)), Outcome(Player(0), Exactly(1))],
                con![Outcome(Player(2), Exactly(2)), Outcome(Player(1), Exactly(0)), Outcome(Player(0), Exactly(2))],
                con![Outcome(Player(2), Exactly(2)), Outcome(Player(1), Exactly(0)), Outcome(Player(0), AtLeast(3))],
                con![Outcome(Player(2), Exactly(2)), Outcome(Player(1), Exactly(1)), Outcome(Player(0), Exactly(0))],
                con![Outcome(Player(2), Exactly(2)), Outcome(Player(1), Exactly(1)), Outcome(Player(0), Exactly(1))],
                con![Outcome(Player(2), Exactly(2)), Outcome(Player(1), Exactly(1)), Outcome(Player(0), Exactly(2))],
                con![Outcome(Player(2), Exactly(2)), Outcome(Player(1), Exactly(1)), Outcome(Player(0), AtLeast(3))],
                con![Outcome(Player(2), Exactly(2)), Outcome(Player(1), Exactly(2)), Outcome(Player(0), Exactly(0))],
                con![Outcome(Player(2), Exactly(2)), Outcome(Player(1), Exactly(2)), Outcome(Player(0), Exactly(1))],
                con![Outcome(Player(2), Exactly(2)), Outcome(Player(1), Exactly(2)), Outcome(Player(0), Exactly(2))],
                con![Outcome(Player(2), Exactly(2)), Outcome(Player(1), Exactly(2)), Outcome(Player(0), AtLeast(3))],
                con![Outcome(Player(2), Exactly(2)), Outcome(Player(1), AtLeast(3))],
                // ---
                con![Outcome(Player(2), AtLeast(3))]
            ],
            dis_cons
        );
    }

    #[test]
    fn transform_2_players_1_3() {
        let dis_cons = transform(2, 1, 3);
        println!("{dis_cons}");
        assert_eq!(
            dis![
                con![Outcome(Player(1), Exactly(0)), Outcome(Player(0), Exactly(1))],
                con![Outcome(Player(1), Exactly(0)), Outcome(Player(0), Exactly(2))],
                con![Outcome(Player(1), Exactly(0)), Outcome(Player(0), AtLeast(3))],
                con![Outcome(Player(1), Exactly(1)), Outcome(Player(0), Exactly(0))],
                con![Outcome(Player(1), Exactly(1)), Outcome(Player(0), Exactly(1))],
                con![Outcome(Player(1), Exactly(1)), Outcome(Player(0), Exactly(2))],
                con![Outcome(Player(1), Exactly(1)), Outcome(Player(0), AtLeast(3))],
                con![Outcome(Player(1), Exactly(2)), Outcome(Player(0), Exactly(0))],
                con![Outcome(Player(1), Exactly(2)), Outcome(Player(0), Exactly(1))],
                con![Outcome(Player(1), Exactly(2)), Outcome(Player(0), Exactly(2))],
                con![Outcome(Player(1), Exactly(2)), Outcome(Player(0), AtLeast(3))],
                con![Outcome(Player(1), AtLeast(3))]
            ],
            dis_cons
        );
    }

    #[test]
    fn transform_2_players_3_3() {
        let dis_cons = transform(2, 3, 3);
        println!("{dis_cons}");
        assert_eq!(
            dis![
                con![Outcome(Player(1), Exactly(0)), Outcome(Player(0), AtLeast(3))],
                con![Outcome(Player(1), Exactly(1)), Outcome(Player(0), Exactly(2))],
                con![Outcome(Player(1), Exactly(1)), Outcome(Player(0), AtLeast(3))],
                con![Outcome(Player(1), Exactly(2)), Outcome(Player(0), Exactly(1))],
                con![Outcome(Player(1), Exactly(2)), Outcome(Player(0), Exactly(2))],
                con![Outcome(Player(1), Exactly(2)), Outcome(Player(0), AtLeast(3))],
                con![Outcome(Player(1), AtLeast(3))]
            ],
            dis_cons
        );
    }

    #[test]
    fn transform_2_players_3_4() {
        let dis_cons = transform(2, 3, 4);
        println!("{dis_cons}");
        assert_eq!(
            dis![
                con![Outcome(Player(1), Exactly(0)), Outcome(Player(0), Exactly(3))],
                con![Outcome(Player(1), Exactly(0)), Outcome(Player(0), AtLeast(4))],
                con![Outcome(Player(1), Exactly(1)), Outcome(Player(0), Exactly(2))],
                con![Outcome(Player(1), Exactly(1)), Outcome(Player(0), Exactly(3))],
                con![Outcome(Player(1), Exactly(1)), Outcome(Player(0), AtLeast(4))],
                con![Outcome(Player(1), Exactly(2)), Outcome(Player(0), Exactly(1))],
                con![Outcome(Player(1), Exactly(2)), Outcome(Player(0), Exactly(2))],
                con![Outcome(Player(1), Exactly(2)), Outcome(Player(0), Exactly(3))],
                con![Outcome(Player(1), Exactly(2)), Outcome(Player(0), AtLeast(4))],
                con![Outcome(Player(1), Exactly(3)), Outcome(Player(0), Exactly(0))],
                con![Outcome(Player(1), Exactly(3)), Outcome(Player(0), Exactly(1))],
                con![Outcome(Player(1), Exactly(3)), Outcome(Player(0), Exactly(2))],
                con![Outcome(Player(1), Exactly(3)), Outcome(Player(0), Exactly(3))],
                con![Outcome(Player(1), Exactly(3)), Outcome(Player(0), AtLeast(4))],
                con![Outcome(Player(1), AtLeast(4))]
            ],
            dis_cons
        );
    }

    #[test]
    fn transform_player_0_2() {
        let dis_cons = for_player(&Player(0), 0, 2);
        assert_eq!(
            dis![
                con![Outcome(Player(0), Exactly(0))],
                con![Outcome(Player(0), Exactly(1))],
                con![Outcome(Player(0), AtLeast(2))]
            ],
            dis_cons
        );
    }

    #[test]
    fn transform_player_1_2() {
        let dis_cons = for_player(&Player(0), 1, 2);
        assert_eq!(
            dis![
                con![Outcome(Player(0), Exactly(1))],
                con![Outcome(Player(0), AtLeast(2))],
            ],
            dis_cons
        );
    }

    #[test]
    fn transform_player_2_2() {
        let dis_cons = for_player(&Player(0), 2, 2);
        assert_eq!(
            dis![
                con![Outcome(Player(0), AtLeast(2))]
            ],
            dis_cons
        );
    }

    #[test]
    #[should_panic(expected="min_target_goals (3) cannot exceed goal_cap (2)")]
    fn transform_player_too_many_goals() {
        for_player(&Player(0), 3, 2);
    }
}
