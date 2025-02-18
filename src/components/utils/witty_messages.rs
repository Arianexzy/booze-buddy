use rand::seq::IndexedRandom;

pub struct WittyMessageBank {
    messages: Vec<&'static str>,
}

impl WittyMessageBank {
    pub fn new() -> Self {
        Self {
            messages: vec![
                "Another one bites the dust! 🍺",
                "Your future self is already mad at you! 😈",
                "SHOTS SHOTS SHOTS! 🥃",
                "Drunk you is taking over! 🎉",
                "Tomorrow's problem! 🤷",
                "Your liver just filed for divorce! 💔",
                "Text your ex! JK DON'T DO IT! 📱",
                "Water is for cowards! 💪",
                "Future hangover loading... 🔄",
                "Your Uber driver's gonna love this! 🚗",
                "Dance floor's calling your name! 💃",
                "Time for drunk food! 🌮",
                "You're gonna regret this tomorrow! 😅",
                "WOOOOOOOOO! 🎉",
                "Is the room spinning or is it just you? 😵‍💫",
                "Your dignity left the chat! 👋",
                "Making memories you won't remember! 📸",
                "Bad decision time! 😎",
                "Plot twist: You're the drunk friend! 🤪",
                "Ordering food for the whole bar! 💳",
            ],
        }
    }
    
    pub fn get_random_message() -> String {
        let bank = Self::new();
        bank.messages
            .choose(&mut rand::rng())
            .unwrap_or(&"¯\\_(ツ)_/¯")
            .to_string()
    }
    
}