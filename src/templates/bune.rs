use super::*;

#[derive(Boilerplate, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct BuneHtml {
  pub(crate) entry: BuneEntry,
  pub(crate) id: BuneId,
  pub(crate) mintable: bool,
  pub(crate) inscription: Option<InscriptionId>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct BuneEntryJson {
  pub(crate) burned: u128,
  pub(crate) divisibility: u8,
  pub(crate) etching: Txid,
  pub(crate) mint: Option<Terms>,
  pub(crate) mints: u128,
  pub(crate) number: u64,
  pub(crate) bune: SpacedBune,
  pub(crate) supply: u128,
  pub(crate) symbol: Option<char>,
  pub(crate) timestamp: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct BuneJson {
  pub(crate) entry: BuneEntryJson,
  pub(crate) id: BuneId,
  pub(crate) mintable: bool,
  pub(crate) inscription: Option<InscriptionId>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct BuneOutputJson {
  pub(crate) bune: SpacedBune,
  pub(crate) balances: Pile,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct BuneAddressJson {
  pub(crate) bunes: Vec<BuneBalance>,
  pub(crate) total_bunes: usize,
  pub(crate) total_elements: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct BuneBalance {
  pub(crate) bune: SpacedBune,
  pub(crate) divisibility: u8,
  pub(crate) symbol: Option<char>,
  pub(crate) total_balance: u128,
  pub(crate) total_outputs: u128,
  pub(crate) balances: Vec<BuneOutput>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct BuneOutput {
  pub(crate) txid: Txid,
  pub(crate) vout: u32,
  pub(crate) script: Script,
  pub(crate) shibes: u64,
  pub(crate) balance: u128,
}

impl PageContent for BuneHtml {
  fn title(&self) -> String {
    format!("Bune {}", self.entry.spaced_bune())
  }
}

#[cfg(test)]
mod tests {
  use {super::*, crate::bunes::Bune};

  #[test]
  fn display() {
    assert_regex_match!(
      BuneHtml {
        entry: BuneEntry {
          burned: 123456789123456789,
          divisibility: 9,
          etching: Txid::all_zeros(),
          number: 25,
          bune: Bune(u128::max_value()),
          supply: 123456789123456789,
          symbol: Some('$'),
          mint: Some(MintEntry {
            end: Some(11),
            limit: Some(1000000001),
            deadline: Some(7),
          }),
          end: Some(11),
          timestamp: 0,
        },
        id: BuneId {
          height: 10,
          index: 9,
        },
        inscription: Some(InscriptionId {
          txid: Txid::all_zeros(),
          index: 0,
        }),
      },
      r"<h1>BCGDENLQRQWDSLRUGSNLBTMFIJAV</h1>
<iframe .* src=/preview/0{64}i0></iframe>
<dl>
  <dt>id</dt>
  <dd>10:9</dd>
  <dt>number</dt>
  <dd>25</dd>
  <dt>supply</dt>
  <dd>\$123456789.123456789</dd>
  <dt>burned</dt>
  <dd>\$123456789.123456789</dd>
  <dt>divisibility</dt>
  <dd>9</dd>
  <dt>open etching end</dt>
  <dd><a href=/block/11>11</a></dd>
  <dt>open etching limit</dt>
  <dd>\$1.000000001</dd>
  <dt>symbol</dt>
  <dd>\$</dd>
  <dt>etching</dt>
  <dd><a class=monospace href=/tx/0{64}>0{64}</a></dd>
  <dt>inscription</dt>
  <dd><a class=monospace href=/inscription/0{64}i0>0{64}i0</a></dd>
</dl>
"
    );
  }
}
