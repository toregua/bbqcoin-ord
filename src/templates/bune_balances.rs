use super::*;

#[derive(Boilerplate, Debug, PartialEq, Serialize, Deserialize)]
pub struct BuneBalancesHtml {
  pub balances: BTreeMap<SpacedBune, BTreeMap<OutPoint, u128>>,
}

impl PageContent for BuneBalancesHtml {
  fn title(&self) -> String {
    "Bune Balances".to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const BUNE: u128 = 99246114928149462;

  #[test]
  fn display_bune_balances() {
    let balances: BTreeMap<Bune, BTreeMap<OutPoint, u128>> = vec![
      (
        Bune(BUNE),
        vec![(
          OutPoint {
            txid: txid(1),
            vout: 1,
          },
          1000,
        )]
        .into_iter()
        .collect(),
      ),
      (
        Bune(BUNE + 1),
        vec![(
          OutPoint {
            txid: txid(2),
            vout: 2,
          },
          12345678,
        )]
        .into_iter()
        .collect(),
      ),
    ]
    .into_iter()
    .collect();

    assert_regex_match!(
      BuneBalancesHtml { balances }.to_string(),
      "<h1>Bune Balances</h1>
<table>
  <tr>
    <th>bune</th>
    <th>balances</th>
  </tr>
  <tr>
    <td><a href=/bune/AAAAAAAAAAAAA>.*</a></td>
    <td>
      <table>
        <tr>
          <td class=monospace>
            <a href=/output/1{64}:1>1{64}:1</a>
          </td>
          <td class=monospace>
            1000
          </td>
        </tr>
      </table>
    </td>
  </tr>
  <tr>
    <td><a href=/bune/AAAAAAAAAAAAB>.*</a></td>
    <td>
      <table>
        <tr>
          <td class=monospace>
            <a href=/output/2{64}:2>2{64}:2</a>
          </td>
          <td class=monospace>
            12345678
          </td>
        </tr>
      </table>
    </td>
  </tr>
</table>
"
      .unindent()
    );
  }
}
