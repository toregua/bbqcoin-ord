use super::*;

#[derive(Boilerplate)]
pub(crate) struct BunesHtml {
  pub(crate) entries: Vec<(BuneId, BuneEntry)>,
}

impl PageContent for BunesHtml {
  fn title(&self) -> String {
    "Bunes".to_string()
  }
}
