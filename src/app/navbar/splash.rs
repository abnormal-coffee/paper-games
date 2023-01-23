use tinyrand::{self, RandRange, Wyrand};

static SPLASHES: [&str; 5] = [
    "Splash 1",
    "Splash 2",
    "Splash 3",
    "Splash 4",
    "Splash 5",
];

pub fn splash(rng: u32) -> String {
    return SPLASHES[rng as usize].to_string();
}

// Fix this so it actually generates a random spash every refresh