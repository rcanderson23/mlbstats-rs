use mlbstats::{games::GameTypes, stats::stats_leaders::StatsLeadersResponse};
use tabled::{
    settings::{panel::Header, width::MinWidth},
    Table, Tabled,
};

use crate::{commands::LeadersOpts, error::CLIError};

#[derive(Tabled)]
struct StatRank {
    #[tabled(rename = "Rank")]
    pub rank: u32,
    #[tabled(rename = "Value")]
    pub value: String,
    #[tabled(rename = "Name")]
    pub name: String,
}

fn create_leaders_table(stat: &str, stat_ranks: &Vec<StatRank>) -> Table {
    Table::new(stat_ranks)
        .with(MinWidth::new(50))
        .with(Header::new(stat))
        .to_owned()
}

pub async fn league_leaders(opts: LeadersOpts) -> Result<(), CLIError> {
    let stat = opts.stat.to_string();
    let leaders_res = get_leaders(opts).await?;
    for leaders in leaders_res.league_leaders {
        let stat_leaders: Vec<StatRank> = leaders
            .leaders
            .iter()
            .map(|leader| StatRank {
                rank: leader.rank,
                value: leader.value.clone(),
                name: leader.person.full_name.clone(),
            })
            .collect();
        println!("{}", create_leaders_table(&stat, &stat_leaders));
    }
    Ok(())
}

async fn get_leaders(opts: LeadersOpts) -> Result<StatsLeadersResponse, CLIError> {
    let client = mlbstats::client::Client::new();
    let stat_group = opts.stat.stat_group();

    let leaders_resp = client
        .stats_leaders(
            vec![opts.stat.into()],
            vec![GameTypes::R],
            Some(stat_group),
            Some(opts.limit),
            opts.league.map(|l| vec![l]),
        )
        .await?;

    Ok(leaders_resp)
}
