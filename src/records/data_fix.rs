use crate::{error::IGCError::DataFixInitError, StrWrapper};
use crate::records::util::Time;
use crate::Result;

#[cfg(feature = "serde")] use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug, Clone)]
pub struct DataFix {
    pub time: Time,
    pub content: StrWrapper,
}

impl DataFix {
    pub(crate) fn parse(line: &str) -> Result<Self> {
        if line.len() < 7 { return Err(DataFixInitError(format!("'{line}' is too short to be parsed as a data fix"))) };
        let time = Time::parse(&line[1..7])?;
        let content = line[7..].to_string().into();
        Ok(Self {time, content})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_fix_parsed_correctly() {
        let data_fix = DataFix::parse("F160240Hello, world!").unwrap();
        assert_eq!(data_fix.time, Time::from_hms(16, 2, 40).unwrap());
        assert_eq!(data_fix.content, "Hello, world!".to_string().into());
    }

    #[test]
    fn too_short() {
        assert!(DataFix::parse("F160").is_err())
    }
}