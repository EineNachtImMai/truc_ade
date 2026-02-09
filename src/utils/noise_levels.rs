pub enum WindowPosition {
    Open,
    Closed,
}

pub enum AllowedActivities {
    QuietPlaying(WindowPosition),
    LoudPlaying(WindowPosition),
    LoudPlayingAndBattery(WindowPosition),
}

impl AllowedActivities {
    fn enum_index(&self) -> u8 {
        match *self {
            AllowedActivities::QuietPlaying(WindowPosition::Closed) => 1,
            AllowedActivities::QuietPlaying(WindowPosition::Open) => 2,
            AllowedActivities::LoudPlaying(WindowPosition::Closed) => 3,
            AllowedActivities::LoudPlaying(WindowPosition::Open) => 4,
            AllowedActivities::LoudPlayingAndBattery(WindowPosition::Closed) => 5,
            AllowedActivities::LoudPlayingAndBattery(WindowPosition::Open) => 6,
        }
    }
}
impl Ord for AllowedActivities {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.enum_index().cmp(&other.enum_index())
    }
}
impl PartialOrd for AllowedActivities {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for AllowedActivities {
    fn eq(&self, other: &Self) -> bool {
        self.enum_index() == other.enum_index()
    }
}
impl Eq for AllowedActivities {}
