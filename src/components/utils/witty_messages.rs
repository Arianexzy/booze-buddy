use rand::seq::IndexedRandom;

pub struct WittyMessageBank {
    active_session_messages: Vec<&'static str>,
    end_session_messages: Vec<&'static str>,
}

impl WittyMessageBank {
    pub fn new() -> Self {
        Self {
            active_session_messages: vec![
                "Another one bites the dust! 🍺",
                "Drunk you is tomorrow's problem",
                "You're not drunk, you're just speaking cursive",
                "Your liver just filed for divorce! 💔",
                "Text your ex!",
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
                "What seems to be the officer, problem?",
                "I thought this was America",
                "Kick some ass, sea bass",
            ],
            end_session_messages: vec![
                "Nighty night sweet tits",
                "Lay your weary head, you gorgeous legend",
                "Sweet dreams, you boozed up beast",
                "Goodnight, you glorious drunk",
                "See ya, you magnificent piss-artist",
                "Off to bed now, you sexy lush",
                "Lights out, you booze-hound",
                "Sweet dreams, you magnificent bastard",
                "Sleep well, you beautiful, drunken legend",
            ],
        }
    }

    pub fn get_random_active_session_message() -> String {
        let bank = Self::new();
        bank.active_session_messages
            .choose(&mut rand::rng())
            .unwrap_or(&"¯\\_(ツ)_/¯")
            .to_string()
    }

    pub fn get_random_end_session_message() -> String {
        let bank = Self::new();
        bank.end_session_messages
            .choose(&mut rand::rng())
            .unwrap_or(&"¯\\_(ツ)_/¯")
            .to_string()
    }
}
