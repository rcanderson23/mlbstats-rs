# mlbstats-rs 

Client for using the MLB Stats API.

## Description

Async client for accessing the MLB Stats API. This library is alpha quality and there is no public documentation for the MLB API, expect bugs. Contributions welcome.

## Usage

### Example

```rust
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
```

