use crate::domain::dynamic_query::utils::get_fragment_with_values;

use super::fragment_like::{FragmentLike, FragmentValue};
use super::utils::{create_parameter_placeholder, ToSqlExtend};

// pub struct Fragment<F: Fn() -> Vec<FragmentValue>> {
pub struct Fragment<'d> {
  fragment_string: String,
  parameter_values: Vec<FragmentValue<'d>>,
}

// impl<F> Fragment<F>
// where
//   F: Fn() -> Vec<FragmentValue>,
impl<'d> Fragment<'d> {
  pub fn new<S>(fragment_string: S, parameter_values: Vec<FragmentValue<'d>>) -> Self
  where
    S: Into<String>,
  {
    Self {
      fragment_string: fragment_string.into(),
      parameter_values,
    }
  }
}

impl<'d> FragmentLike<'d> for Fragment<'d> {
  fn get_parameterized_fragment(&self) -> Option<String> {
    let mut parameterized_fragment = self.fragment_string.replace("?", "{}");

    for value in &self.parameter_values {
      match value {
        FragmentValue::Fragment(fragment) => {
          parameterized_fragment = parameterized_fragment.replacen(
            "{}",
            fragment
              .get_parameterized_fragment()
              .as_ref()
              .map(|str| str as &str)
              .unwrap_or(""),
            1,
          );
        }
        _ => {
          parameterized_fragment =
            parameterized_fragment.replacen("{}", &create_parameter_placeholder(&value, "?"), 1);
        }
      }
    }

    Some(parameterized_fragment)
  }

  fn get_parameter_count(&self) -> Option<usize> {
    let mut count = 0;

    for value in &self.parameter_values {
      match value {
        FragmentValue::List(list) => count += list.len(),
        FragmentValue::Value(_) => count += 1,
        FragmentValue::Fragment(fragment) => {
          count += fragment.get_parameter_count().unwrap_or(0);
        }
      }
    }

    Some(count)
  }

  fn get_parameterized_values(&self) -> Option<Vec<&'d dyn ToSqlExtend>> {
    let mut parameterized_values: Vec<&'d dyn ToSqlExtend> = vec![];

    for value in &self.parameter_values {
      match value {
        FragmentValue::List(list) => {
          parameterized_values.extend(list);
        }
        FragmentValue::Value(value) => parameterized_values.push(*value),
        FragmentValue::Fragment(fragment) => {
          if let Some(values) = fragment.get_parameterized_values() {
            parameterized_values.extend(values);
          }
        }
      }
    }

    Some(parameterized_values)
  }

  fn get_fragment_with_values(&self) -> Option<String> {
    get_fragment_with_values(self)
  }
}
