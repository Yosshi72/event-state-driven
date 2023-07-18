#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub enum Event {
    AttackedByEnemy,
    GetMash,
    GetFlower,
    GetStar,
    TimePassed,
}