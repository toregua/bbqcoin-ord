use super::*;

#[derive(Copy, Clone, Debug, PartialEq, Ord, PartialOrd, Eq, Hash)]
pub struct SpacedBune {
  pub(crate) bune: Bune,
  pub(crate) spacers: u32,
}

impl SpacedBune {
  pub fn new(bune: Bune, spacers: u32) -> Self {
    Self { bune, spacers }
  }
}

impl FromStr for SpacedBune {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut bune = String::new();
    let mut spacers = 0u32;

    for c in s.chars() {
      match c {
        'A'..='Z' => bune.push(c),
        '.' | '•' => {
          let flag = 1 << bune.len().checked_sub(1).context("leading spacer")?;
          if spacers & flag != 0 {
            bail!("double spacer");
          }
          spacers |= flag;
        }
        _ => bail!("invalid character"),
      }
    }

    if 32 - spacers.leading_zeros() >= bune.len().try_into().unwrap() {
      bail!("trailing spacer")
    }

    Ok(SpacedBune {
      bune: bune.parse()?,
      spacers,
    })
  }
}

impl Display for SpacedBune {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    let bune = self.bune.to_string();

    for (i, c) in bune.chars().enumerate() {
      write!(f, "{c}")?;

      if i < bune.len() - 1 && self.spacers & 1 << i != 0 {
        write!(f, "•")?;
      }
    }

    Ok(())
  }
}

impl Serialize for SpacedBune {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.collect_str(self)
  }
}

impl<'de> Deserialize<'de> for SpacedBune {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    Ok(DeserializeFromStr::deserialize(deserializer)?.0)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn display() {
    assert_eq!("A.B".parse::<SpacedBune>().unwrap().to_string(), "A•B");
    assert_eq!("A.B.C".parse::<SpacedBune>().unwrap().to_string(), "A•B•C");
  }

  #[test]
  fn from_str() {
    #[track_caller]
    fn case(s: &str, bune: &str, spacers: u32) {
      assert_eq!(
        s.parse::<SpacedBune>().unwrap(),
        SpacedBune {
          bune: bune.parse().unwrap(),
          spacers
        },
      );
    }

    assert_eq!(
      ".A".parse::<SpacedBune>().unwrap_err().to_string(),
      "leading spacer",
    );

    assert_eq!(
      "A..B".parse::<SpacedBune>().unwrap_err().to_string(),
      "double spacer",
    );

    assert_eq!(
      "A.".parse::<SpacedBune>().unwrap_err().to_string(),
      "trailing spacer",
    );

    assert_eq!(
      "Ax".parse::<SpacedBune>().unwrap_err().to_string(),
      "invalid character",
    );

    case("A.B", "AB", 0b1);
    case("A.B.C", "ABC", 0b11);
    case("A•B", "AB", 0b1);
    case("A•B•C", "ABC", 0b11);
  }

  #[test]
  fn serde() {
    let spaced_bune = SpacedBune {
      bune: Bune(26),
      spacers: 1,
    };
    let json = "\"A•A\"";
    assert_eq!(serde_json::to_string(&spaced_bune).unwrap(), json);
    assert_eq!(
      serde_json::from_str::<SpacedBune>(json).unwrap(),
      spaced_bune
    );
  }
}
