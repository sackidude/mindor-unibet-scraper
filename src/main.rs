mod bet365;
mod unibet;

const URL: &'static str = "https://eu-offering-api.kambicdn.com/offering/v2018/ubse/betoffer/event/1020336490.json?lang=sv_SE&market=SE&client_id=2&channel_id=1&ncid=1711718705705&includeParticipants=true";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let unibet_data = unibet::get(URL).await?;
    let bet365_data = bet365::get(URL).await?;

    let event_filename = format!("{}.csv", &unibet_data.events[0].name);
    let file = std::fs::File::create(&event_filename)?;

    let mut writer = csv::WriterBuilder::new().delimiter(b'\t').from_writer(file);

    println!("Writing data...");
    writer.write_record(vec!["Offer", "Unibet", "bet365"])?;
    writer.flush()?;
    for offer in unibet_data.bet_offers {
        for outcome in offer.outcomes {
            if let Some(odds) = outcome.odds {
                writer.write_record(vec![
                    format!(
                        "{}>{}",
                        offer.criterion.english_label, outcome.english_label
                    ),
                    odds.to_string(),
                ])?;
            }
        }
    }
    writer.flush()?;
    println!("Wrote to: {}", &event_filename);

    Ok(())
}
