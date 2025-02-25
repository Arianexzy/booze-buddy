use rand::seq::IndexedRandom;

pub struct WittyMessageBank {
    active_session_messages: Vec<&'static str>,
    end_session_messages: Vec<&'static str>,
}

impl WittyMessageBank {
    pub fn new() -> Self {
        Self {
            active_session_messages: vec![
                "Drunk you is tomorrow's problem",
                "You're not drunk, you're just speaking cursive",
                "Text your ex!",
                "Water sucks, it really really sucks",
                "Future hangover loading... 🔄",
                "Your dignity left the chat 👋",
                "Making memories you won't remember 📸",
                "Bad decision time 😎",
                "What seems to be the officer, problem?",
                "I thought this was America",
                "Kick some ass, sea bass",
                "That's it, dull them senses",
                "Time to go hard or go home hard",
                "Your pants still on?",
                "Drunk you thinks everyone wants to see your nipples",
                "Send nudes!",
                "You're not a mess, you're a debaucherous masterpiece",
                "You can't handle the proof!",
                "The first rule of Drink Club: you don't remember Drink Club" ,
                "You’re not an alcoholic, you’re just collecting experiences",
                "You’re not drunk, you’re just emotionally hydrated",
                "You’re not drunk, you’re just somehow better at everything",
                "You’re not drunk, you’re just temporarily awesome",
                "You're not drunk, you're just funner to be around",
                "Drink now, apologize tomorrow",
                "Drinking’s the reason your pants are optional",
                "You could totally kick that guy's ass",
                "Your sex appeal is up 200%!",
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
                "Go home and cuddle that toilet, you beautiful disaster",
                "Sweet dreams, you hot mess express",
                "Sleep tight, don't forget to delete your browser history",
                "Go to bed before you send nudes to your family group chat",
                "Sleep it off, you magnificent booze-bag",
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
