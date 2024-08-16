use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub score: Option<usize>,
    pub deaths: Option<usize>,
    pub ping: Option<Duration>,
}

pub trait IntoPlayer {
    fn name(&self) -> String;

    fn score(&self) -> Option<usize> { None }

    fn deaths(&self) -> Option<usize> { None }

    fn ping(&self) -> Option<Duration> { None }

    fn into_player(self) -> Player
    where Self: Sized {
        Player {
            name: self.name(),
            score: self.score(),
            deaths: self.deaths(),
            ping: self.ping(),
        }
    }
}
