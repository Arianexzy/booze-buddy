use rand::seq::IndexedRandom;

pub struct WittyMessageBank {
    messages: Vec<&'static str>,
}

impl WittyMessageBank {
    pub fn new() -> Self {
        Self {
            messages: vec![
                "Another one bites the dust! 🍺",
                "Drunk you is tomorrow's problem",
                "You're not drunk, you're just speaking cursive",
                "Your liver just filed for divorce! 💔",
                "Text your ex! JK DON'T DO IT! 📱",
                "Water sucks, it really really sucks",
                "Future hangover loading... 🔄",
                "Your Uber driver's gonna love this! 🚗",
                "Time for drunk food! 🌮",
                "You're gonna regret this tomorrow! 😅",
                "WOOOOOOOOO! 🎉",
                "Is the room spinning or is it just you? 😵‍💫",
                "Your dignity left the chat! 👋",
                "Making memories you won't remember! 📸",
                "Bad decision time 😎",
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
