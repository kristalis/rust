use crate::*;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolidayLibrary {
  #[serde(rename = "holiday-lib")]
  pub holiday_lib: HolidayLibTag
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolidayLibTag { pub uid: String, pub frequencies: Frequency }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frequency { pub frequency: Vec<FrequencyAttr> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrequencyAttr { pub recommended: String, pub date: String }



/*
  Builds holiday library structure
*/
pub fn get_holiday_library(dir: &str, filename: &str) -> HolidayLibrary {
  let filepath = format!("{dir}/{filename}");
  let data = fs::read_to_string(filepath).unwrap();
  let item: HolidayLibrary = serde_xml_rs::from_str(data.as_str()).unwrap();
  item
}

/*
  Return recommendable holiday book frequency
*/
pub fn is_recommendable(input_date: &str, frequencies: &Vec<FrequencyAttr>) -> Option<usize> {
  for frequency in frequencies {
    if input_date == &frequency.date {
      return Some(frequency.recommended.parse::<usize>().unwrap())
    }
  }

  None
}
