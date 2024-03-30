#![allow(dead_code)]

const URL: &'static str = "https://eu-offering-api.kambicdn.com/offering/v2018/ubse/betoffer/event/1020336490.json?lang=sv_SE&market=SE&client_id=2&channel_id=1&ncid=1711718705705&includeParticipants=true";

#[derive(serde::Deserialize)]
struct Criterion {
    id: u32,
    label: String,
    #[serde(rename = "englishLabel")]
    english_label: String,
    order: serde_json::Value, // Vec<String>, // TODO!: FIX THIS?
    #[serde(rename = "occurenceType")]
    occurence_type: Option<String>,
    lifetime: Option<String>,
}

#[derive(serde::Deserialize)]
struct BetOfferType {
    id: u32,
    name: String,
    #[serde(rename = "englishName")]
    english_name: String,
}

#[derive(serde::Deserialize)]
struct Outcome {
    id: u32,
    label: String,
    #[serde(rename = "englishLabel")]
    english_label: String,
    odds: Option<u32>,
    participant: Option<String>,
    #[serde(rename = "type")]
    bet_type: String,
    #[serde(rename = "betOfferId")]
    bet_offer_id: u32,
    #[serde(rename = "changedDate")]
    changed_date: String,
    #[serde(rename = "participantId")]
    participant_id: Option<u32>,
    #[serde(rename = "oddsFractional")]
    odds_fractional: Option<String>,
    #[serde(rename = "oddsAmerican")]
    odds_american: Option<String>,
    status: String,
    #[serde(rename = "cashOutStatus")]
    cash_out_status: String,
}

#[derive(serde::Deserialize)]
struct BetOffer {
    id: u32,
    closed: String,
    criterion: Criterion,
    #[serde(rename = "betOfferType")]
    bet_offer_type: BetOfferType,
    #[serde(rename = "eventId")]
    event_id: u32,
    outcomes: Vec<Outcome>,
    tags: Vec<String>,
}

#[derive(serde::Deserialize)]
struct Path {
    id: u32,
    name: String,
    #[serde(rename = "englishLabel")]
    english_label: Option<String>,
    #[serde(rename = "termKey")]
    term_key: String,
}

#[derive(serde::Deserialize)]
struct Participant {
    #[serde(rename = "participantId")]
    participant_id: u32,
    name: String,
    scratched: bool,
    #[serde(rename = "nonRunner")]
    non_runner: bool,
    home: bool,
    #[serde(rename = "participantType")]
    participant_type: String,
}

#[derive(serde::Deserialize)]
struct Event {
    id: u32,
    name: String,
    #[serde(rename = "nameDelimiter")]
    name_delimiter: String,
    #[serde(rename = "englishName")]
    english_name: String,
    #[serde(rename = "homeName")]
    home_name: String,
    #[serde(rename = "awayName")]
    away_name: String,
    start: String,
    group: String,
    #[serde(rename = "groupId")]
    group_id: u32,
    path: Vec<Path>,
    #[serde(rename = "nonLiveBoCount")]
    non_live_bo_count: u32,
    sport: String,
    tags: Vec<String>,
    state: String,
    participants: Vec<Participant>,
}

#[derive(serde::Deserialize)]
struct Odd {
    decimal: u32,
    american: String,
    fractional: String,
}

#[derive(serde::Deserialize)]
struct WTFOutcome {
    id: u32,
    #[serde(rename = "eventId")]
    event_id: u32,
    #[serde(rename = "betOfferId")]
    bet_offer_id: u32,
}

#[derive(serde::Deserialize)]
struct OperationOutcome {
    operation: String,
    groups: Vec<WTFOutcome>,
}

#[derive(serde::Deserialize)]
struct OperationGroup {
    operation: String,
    groups: Vec<OperationOutcome>,
}

#[derive(serde::Deserialize)]
struct OddsGroup {
    odds: Odd,
    groups: Vec<OperationGroup>,
}

#[derive(serde::Deserialize)]
struct Selection {
    #[serde(rename = "selectionId")]
    selection_id: u32,
    label: Vec<String>,
    status: String,
    combination: Vec<OddsGroup>,
    tags: Vec<String>,
}

#[derive(serde::Deserialize)]
struct Response {
    #[serde(rename = "betOffers")]
    bet_offers: Vec<BetOffer>,
    events: Vec<Event>,
    #[serde(rename = "prePacks")]
    pre_packs: serde_json::Value, // Vec<PrePack>, // NOTE: This is useless?
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Fetching URL...");
    let response = reqwest::get(URL).await?;
    println!("Parsing data...");
    let data: Response = response.json().await?;
    println!("Successfully parsed.");

    for offer in data.bet_offers {}

    Ok(())
}
