use crate::components::counter_btn::Button;
use leptos::*;

// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::Card;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: Card = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Card {
    pub ability: Option<i64>,
    pub armor: Option<i64>,
    pub characteristics: Option<String>,
    pub clone_army: Option<bool>,
    pub code: String,
    pub dark_side_icons: Option<i64>,
    pub dark_side_text: Option<String>,
    pub defense_value: Option<i64>,
    pub defense_value_name: Option<String>,
    pub deploy: Option<i64>,
    pub destiny: Option<i64>,
    pub episode_1: Option<bool>,
    pub episode_7: Option<bool>,
    pub ferocity: Option<i64>,
    pub first_order: Option<bool>,
    pub force_aptitude: Option<String>,
    pub forfeit: Option<i64>,
    pub gametext: String,
    pub grabber: Option<bool>,
    pub has_errata: bool,
    pub hyperspeed: Option<i64>,
    pub image_url: String,
    pub image_url_2: Option<String>,
    pub independent: Option<bool>,
    pub is_horizontal: Option<bool>,
    pub landspeed: Option<i64>,
    pub light_side_icons: Option<i64>,
    pub light_side_text: Option<String>,
    pub lore: Option<String>,
    pub maneuver: Option<i64>,
    pub mobile: Option<bool>,
    pub model_type: Option<String>,
    pub name: String,
    pub nav_computer: Option<bool>,
    pub permanent_weapon: Option<bool>,
    pub pilot: Option<bool>,
    pub planet: Option<bool>,
    pub politics: Option<i64>,
    pub position: i64,
    pub power: Option<i64>,
    pub presence: Option<bool>,
    pub rarity_code: RarityCode,
    pub republic: Option<bool>,
    pub resistance: Option<bool>,
    pub scomp_link: Option<bool>,
    pub selective: Option<bool>,
    pub separatist: Option<bool>,
    pub set_code: String,
    pub side_code: SideCode,
    pub site_creature: Option<bool>,
    pub site_exterior: Option<bool>,
    pub site_interior: Option<bool>,
    pub site_starship: Option<bool>,
    pub site_underground: Option<bool>,
    pub site_underwater: Option<bool>,
    pub site_vehicle: Option<bool>,
    pub space: Option<bool>,
    pub subtype_code: Option<SubtypeCode>,
    pub system_parsec: Option<i64>,
    pub trade_federation: Option<bool>,
    pub type_code: TypeCode,
    pub uniqueness: String,
    pub warrior: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub enum RarityCode {
    C,
    C1,
    C2,
    C3,
    F,
    #[serde(rename = "PM")]
    Pm,
    #[serde(rename = "PM2")]
    Pm2,
    #[serde(rename = "PM3")]
    Pm3,
    #[serde(rename = "PM5")]
    Pm5,
    #[serde(rename = "PV")]
    Pv,
    R,
    R1,
    R2,
    U,
    U1,
    U2,
    #[serde(rename = "UR")]
    Ur,
    #[serde(rename = "XR")]
    Xr,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideCode {
    Dark,
    Light,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SubtypeCode {
    Alien,
    #[serde(rename = "alien-imperial")]
    AlienImperial,
    #[serde(rename = "alien-rebel")]
    AlienRebel,
    #[serde(rename = "alien-resistance")]
    AlienResistance,
    Artillery,
    Automated,
    Capital,
    Character,
    Combat,
    Creature,
    #[serde(rename = "dark-jedi-master")]
    DarkJediMaster,
    #[serde(rename = "death-star")]
    DeathStar,
    #[serde(rename = "death-star-II")]
    DeathStarIi,
    Droid,
    #[serde(rename = "first-order")]
    FirstOrder,
    Immediate,
    Imperial,
    Jedi,
    #[serde(rename = "jedi-master")]
    JediMaster,
    #[serde(rename = "jedi-master-imperial")]
    JediMasterImperial,
    Lost,
    #[serde(rename = "lost-or-starting")]
    LostOrStarting,
    Mobile,
    Political,
    Rebel,
    #[serde(rename = "rebel-republic")]
    RebelRepublic,
    Republic,
    Resistance,
    Sector,
    Shuttle,
    Site,
    Sith,
    Squadron,
    Starfighter,
    Starship,
    Starting,
    System,
    Transport,
    Used,
    #[serde(rename = "used-or-lost")]
    UsedOrLost,
    #[serde(rename = "used-or-starting")]
    UsedOrStarting,
    Utinni,
    Vehicle,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TypeCode {
    #[serde(rename = "admirals-order")]
    AdmiralsOrder,
    Character,
    Creature,
    #[serde(rename = "defensive-shield")]
    DefensiveShield,
    Device,
    Effect,
    #[serde(rename = "epic-event")]
    EpicEvent,
    Interrupt,
    #[serde(rename = "jedi-test")]
    JediTest,
    Location,
    Objective,
    Podracer,
    Starship,
    Vehicle,
    Weapon,
}


/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            <div class="container">

                <picture>
                    <source
                        srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_pref_dark_RGB.svg"
                        media="(prefers-color-scheme: dark)"
                    />
                    <img
                        src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg"
                        alt="Leptos Logo"
                        height="200"
                        width="400"
                    />
                </picture>

                <h1>"Welcome to Leptos"</h1>

                <div class="buttons">
                    <Button/>
                    <Button increment=5/>
                </div>

            </div>
        </ErrorBoundary>
    }
}
