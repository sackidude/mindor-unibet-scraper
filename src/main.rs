use std::{collections::HashMap, io::Write};

use itertools::Itertools;

mod unibet;

fn odd_num_to_str(num: u32) -> String {
    let mut num_str = num.to_string();
    let len = num_str.len();

    // Check if the number is less than 1000
    if len <= 3 {
        num_str.insert(0, '.');
        println!("WARN!: {}", num);
        return num_str;
    }

    // Calculate the index to insert the dot
    let index = len - 3;

    // Insert the dot at the calculated index
    num_str.insert(index, '.');
    num_str
}

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

    let home_name = unibet_data.events[0].home_name.clone();
    let away_name = unibet_data.events[0].away_name.clone();

    println!("Writing data...");
    let mut unsorted_data = HashMap::<String, String>::new();
    for offer in unibet_data.bet_offers {
        for outcome in offer.outcomes {
            if let Some(odds) = outcome.odds {
                let mut printed = false;
                let odd_str = odd_num_to_str(odds);
                // TODO!: INFO!: WARN!: REFACTOR
                if let Some(participant) = outcome.participant {
                    if !(participant == home_name || participant == away_name) {
                        if outcome.label == "Över" {
                            unsorted_data.insert(
                                format!(
                                    "{} > {} {} [{}]",
                                    offer.criterion.label,
                                    outcome.label,
                                    odd_num_to_str(
                                        outcome.line.unwrap().try_into().unwrap() // WARN!: What is this!!!
                                    ),
                                    participant
                                ),
                                odd_str.clone(),
                            );
                        } else {
                            unsorted_data.insert(
                                format!(
                                    "{} > {} [{}]",
                                    offer.criterion.label, outcome.label, participant
                                ),
                                odd_str.clone(),
                            );
                        }
                        printed = true;
                    }
                }
                if !printed {
                    unsorted_data.insert(
                        format!("{} > {}", offer.criterion.label, outcome.label),
                        odd_str,
                    );
                }
            }
        }
    }
    let mut writer = csv::WriterBuilder::new().delimiter(b'\t').from_writer(file);

    writer.write_record(vec!["Erbjudande", "Unibet"])?;
    writer.flush()?;

    for key in unsorted_data.keys().sorted() {
        writer.write_record(vec![key, &format!("{}", unsorted_data[key])])?
    }

    writer.flush()?;
    println!("Wrote to: {}", &event_filename);

    Ok(())
}
