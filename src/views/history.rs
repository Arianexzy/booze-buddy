use crate::storage::{
    models::{
        Achievement,
        AchievementTier::{Bronze, Gold, Platinum, Silver},
        DrinkType::{Beer, MixedDrink, Shot, Wine},
    },
    storage::drink_history::get_past_sessions,
};
use dioxus::prelude::*;

const HISTORY_VIEW_CSS: Asset = asset!("/assets/styling/history_view.css");

#[component]
pub fn DrinkingHistoryView() -> Element {
    let mut sessions = get_past_sessions().unwrap_or(vec![]);
    sessions.sort_by(|a, b| b.start_time.cmp(&a.start_time));

    rsx! {
        document::Link { rel: "stylesheet", href: HISTORY_VIEW_CSS }
        h1 { class: "view-header", "Past Rockin Nights" }
        div { class: "sessions-container",
            {
                sessions
                    .iter()
                    .map(|session| {
                        let total_drinks = session.total_drinks();
                        let date = session.start_time.unwrap().date_naive();

                        // Drink Type counts
                        let beers = session.count_by(Beer);
                        let wines = session.count_by(Wine);
                        let mixed_drinks = session.count_by(MixedDrink);
                        let shots = session.count_by(Shot);

                        // Drink Icons
                        let beer_icon = Beer.display().icon;
                        let wine_icon = Wine.display().icon;
                        let mixed_drink_icon = MixedDrink.display().icon;
                        let shot_icon = Shot.display().icon;

                        // Achievement counts
                        let achievements = &session.achievements;
                        let bronze_count = achievements.iter().filter(|a| a.tier == Bronze).count();
                        let silver_count = achievements.iter().filter(|a| a.tier == Silver).count();
                        let gold_count = achievements.iter().filter(|a| a.tier == Gold).count();
                        let platinum_count = achievements.iter().filter(|a| a.tier == Platinum).count();

                        // Achievement Icons
                        let bronze_icon = Achievement::display(Bronze).tier_icon;
                        let silver_icon = Achievement::display(Silver).tier_icon;
                        let gold_icon = Achievement::display(Gold).tier_icon;
                        let platinum_icon = Achievement::display(Platinum).tier_icon;

                        let max_bac = format!("{:.3}", session.max_bac);

                        rsx! {
                            div { class: "session-card",

                                h2 { class: "session-date", "{date}"}

                                // Stats
                                div { class: "session-stats",
                                    span {
                                        "data-label": "TOTAL DRINKS",
                                        strong { "{total_drinks}" }
                                    }
                                    span {
                                        "data-label": "MAX BAC",
                                        "data-bac": "{max_bac}",
                                        strong { "{max_bac}" }
                                    }
                                }

                                // Drink Counts
                                div { class: "drink-types",
                                    div { class: "drink-type",
                                        span { class: "drink-icon", "{beer_icon}"}
                                        span { class: "drink-value", "{beers}"}
                                    }
                                    div { class: "drink-type",
                                        span { class: "drink-icon", "{wine_icon}"}
                                        span { class: "drink-value", "{wines}"}
                                    }
                                    div { class: "drink-type",
                                        span { class: "drink-icon", "{mixed_drink_icon}"}
                                        span { class: "drink-value", "{mixed_drinks}"}
                                    }
                                    div { class: "drink-type",
                                        span { class: "drink-icon", "{shot_icon}"}
                                        span { class: "drink-value", "{shots}"}
                                    }
                                }

                                // Achievements
                                div { class: "achievements-section",
                                    div { class: "achievement-tier",
                                        span { class: "tier-icon", "{bronze_icon}" }
                                        span { "{bronze_count}" }
                                    }
                                    div { class: "achievement-tier",
                                        span { class: "tier-icon", "{silver_icon}" }
                                        span { "{silver_count}" }
                                    }
                                    div { class: "achievement-tier",
                                        span { class: "tier-icon", "{gold_icon}" }
                                        span { "{gold_count}" }
                                    }
                                    div { class: "achievement-tier",
                                        span { class: "tier-icon", "{platinum_icon}" }
                                        span { "{platinum_count}" }
                                    }
                                }
                            }
                        }
                    })
            }
        }
    }
}
