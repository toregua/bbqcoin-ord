use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Output {
  pub bunes: BTreeMap<SpacedBune, BTreeMap<OutPoint, u128>>,
}

pub(crate) fn run(options: Options) -> SubcommandResult {
  let index = Index::open(&options)?;

  ensure!(
    index.has_bune_index(),
    "`ord balances` requires index created with `--index-bunes` flag",
  );

  index.update()?;

  Ok(Box::new(Output {
    bunes: index.get_bune_balance_map()?,
  }))
}
