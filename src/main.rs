use std::io::Write;

mod bet365;
mod unibet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("Enter the number at end of UNIBET url address: ");
    std::io::stdout().flush()?;
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;
    let num = buffer.trim();
    let url = format!("https://eu-offering-api.kambicdn.com/offering/v2018/ubse/betoffer/event/{}.json?lang=sv_SE&market=SE&client_id=2&channel_id=1&ncid=1711718705705&includeParticipants=true", num);

    let unibet_data = unibet::get(&url).await?;
    // let bet365_data = bet365::get(&url).await?;

    let event_filename = format!("{}.csv", &unibet_data.events[0].name);
    let file = std::fs::File::create(&event_filename)?;

    let mut writer = csv::WriterBuilder::new().delimiter(b'\t').from_writer(file);

    println!("Writing data...");
    writer.write_record(vec!["Offer", "Unibet", "bet365", "arbitrage"])?;
    writer.flush()?;
    for offer in unibet_data.bet_offers {
        for outcome in offer.outcomes {
            if let Some(odds) = outcome.odds {
                writer.write_record(vec![
                    format!("{} > {}", offer.criterion.label, outcome.label),
                    odds.to_string(),
                    "".to_string(),
                    "".to_string(),
                ])?;
            }
        }
    }
    writer.flush()?;
    println!("Wrote to: {}", &event_filename);

    Ok(())
}
