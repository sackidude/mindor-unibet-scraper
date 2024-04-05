use std::{collections::HashMap, io::Write};

use itertools::Itertools;

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

    let event_filename = format!("{}.csv", &unibet_data.events[0].name);
    let file = std::fs::File::create(&event_filename)?;

    let mut writer = csv::WriterBuilder::new().delimiter(b'\t').from_writer(file);

    println!("Writing data...");
    let mut unsorted_data = HashMap::<String, u32>::new();
    for offer in unibet_data.bet_offers {
        for outcome in offer.outcomes {
            if let Some(odds) = outcome.odds {
                if let Some(participant) = outcome.participant {
                    unsorted_data.insert(
                        format!(
                            "{} > {} [{}]",
                            offer.criterion.label, outcome.label, participant
                        ),
                        odds,
                    );
                } else {
                    unsorted_data.insert(
                        format!("{} > {}", offer.criterion.label, outcome.label),
                        odds,
                    );
                }
            }
        }
    }
    writer.write_record(vec!["Erbjudande", "Unibet"])?;
    writer.flush()?;

    for key in unsorted_data.keys().sorted() {
        writer.write_record(vec![key, &format!("{}", unsorted_data[key])])?
    }

    writer.flush()?;
    println!("Wrote to: {}", &event_filename);

    Ok(())
}
