use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Output {
  pub bunes: BTreeMap<Bune, BuneInfo>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BuneInfo {
  pub block: u64,
  pub burned: u128,
  pub divisibility: u8,
  pub etching: Txid,
  pub height: u64,
  pub id: BuneId,
  pub index: u32,
  pub terms: Option<Terms>,
  pub mints: u128,
  pub number: u64,
  pub premine: u128,
  pub bune: Bune,
  pub spacers: u32,
  pub supply: u128,
  pub symbol: Option<char>,
  pub timestamp: DateTime<Utc>,
  pub turbo: bool,
  pub tx: u32,
}

pub(crate) fn run(options: Options) -> SubcommandResult {
  let index = Index::open(&options)?;

  ensure!(
    index.has_bune_index(),
    "`ord bunes` requires index created with `--index-bunes` flag",
  );

  index.update()?;

  Ok(Box::new(Output {
    bunes: index
      .bunes()?
      .into_iter()
      .map(
        |(
          id,
          entry @ BuneEntry {
            block,
            burned,
            divisibility,
            etching,
            terms,
            mints,
            number,
            premine,
            bune,
            spacers,
            supply,
            symbol,
            timestamp,
            turbo,
          },
        )| {
          (
            bune,
            BuneInfo {
              block,
              burned,
              divisibility,
              etching,
              height: id.height,
              id,
              index: id.index,
              terms,
              mints,
              number,
              premine,
              timestamp: crate::timestamp(timestamp),
              bune,
              spacers,
              supply,
              symbol,
              turbo,
              tx: id.index,
            },
          )
        },
      )
      .collect::<BTreeMap<Bune, BuneInfo>>(),
  }))
}
