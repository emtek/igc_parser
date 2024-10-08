use crate::error::IGCError::TaskInfoInitError;
use crate::records::util::{Coordinate, Date, Time};
use crate::{Result, StrWrapper};
#[cfg(feature = "serde")] use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug, Clone)]
pub enum TaskInfo {
    TaskPoint(TaskPoint),
    DeclarationTime(DeclarationTime),
}

impl TaskInfo {
    pub(crate) fn parse(line: &str) -> Result<Self> {
        if line.len() < 18 { return Err(TaskInfoInitError(format!("'{}' is too short to be parsed as kind of task info record", line))) }
        if line[1..17].chars().all(|c| c.is_numeric()) {
            Ok(TaskInfo::DeclarationTime(DeclarationTime::parse(line)?))
        } else {
            Ok(TaskInfo::TaskPoint(TaskPoint::parse(line)?))
        }
    }
}

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug, Clone)]
pub struct TaskPoint {
    pub coordinate: Coordinate,
    pub name: Option<StrWrapper>,
}

impl TaskPoint {
    pub fn parse(line: &str) -> Result<Self> {
        let coordinate = Coordinate::parse(&line[1..18])?;
        let name = (line.len() != 18).then(|| line[18..].to_string().into());
        Ok(Self { coordinate, name })
    }
}

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug, Clone)]
pub struct DeclarationTime {
    pub date: Date,
    pub time: Time,
    extra: StrWrapper,
}

impl DeclarationTime {
    fn parse(line: &str) -> Result<Self> {
        if line.len() < 23 { return Err(TaskInfoInitError(format!("'{}' is too short to be a declaration time record", line))) }
        let date = Date::parse(&line[1..7])?;
        let time = Time::parse(&line[7..13])?;
        let extra = line[13..].to_string().into();
        Ok(Self { date, time, extra })
    }
}

impl DeclarationTime {
    pub fn get_extra(&self) -> StrWrapper {
        self.extra.clone().into()
    }
}

#[cfg(test)]
mod tests {
    use crate::records::util::{Latitude, Longitude};
    use super::*;

    #[test]
    fn declaration_time_parsed_correctly() {
        if let Ok(TaskInfo::DeclarationTime(decl_time)) = TaskInfo::parse("C070323213339000000000103") {
            assert_eq!(decl_time.time, Time::from_hms(21, 33, 39).unwrap());
            assert_eq!(decl_time.date, Date { d: 7, m: 3, y: 23 })
        } else {
            assert!(false)
        }
    }

    #[test]
    fn declaration_too_short() {
        assert!(TaskInfo::parse("C070323213339000000000").is_err());
        assert!(TaskInfo::parse("C0703232133390").is_err())
    }

    #[test]
    fn task_point_parsed_correctly() {
        if let Ok(TaskInfo::TaskPoint(task_point)) = TaskInfo::parse("C3835269S17609420ETASA Taupo Start A") {
            assert_eq!(task_point.coordinate.latitude, Latitude {
                degrees: 38,
                minutes: 35.269,
                is_north: false,
            });

            assert_eq!(task_point.coordinate.longitude, Longitude {
                degrees: 176,
                minutes: 9.42,
                is_east: true,
            });

            assert_eq!(task_point.name.unwrap(), String::from("TASA Taupo Start A").into())
        } else {
            assert!(false)
        }
    }
}