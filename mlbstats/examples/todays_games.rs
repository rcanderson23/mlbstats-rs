use mlbstats::client::Client;
use chrono::NaiveDate;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let client = Client::new();

    let schedule = client.schedule(None, None, None::<NaiveDate>, None).await?;

    for date in schedule.dates {
        for game in date.games {
            println!("{:?}", game);
        }
    }

    Ok(())
}
