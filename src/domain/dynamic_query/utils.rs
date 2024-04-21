use std::borrow::Cow;

use iter_tools::Itertools;
use postgres::types::ToSql;

use super::fragment_like::{FragmentLike, FragmentValue};

pub fn get_fragment_with_values<'d>(fragment: &dyn FragmentLike<'d>) -> Option<String> {
  let values = fragment.get_parameterized_values().unwrap_or(vec![]);

  fragment
    .get_parameterized_fragment()
    .map(|mut new_fragment| {
      for value in &values {
        new_fragment = new_fragment.replacen("?", &format!("{:?}", &value), 1);
      }

      new_fragment.replace("\\:\\:", "::")
    })
}

pub fn create_parameter_placeholder(value: &FragmentValue, placeholder: &str) -> String {
  create_parameter_placeholder_type(value, placeholder, None)
}

pub fn create_parameter_placeholder_type(
  value: &FragmentValue,
  placeholder: &str,
  type_: Option<&str>,
) -> String {
  let mut new_placeholder = Cow::Borrowed(placeholder);
  let mut count = 1;

  if let Some(type_) = type_ {
    new_placeholder = Cow::Owned(format!("?\\:\\:{}", type_));
  }

  if let FragmentValue::List(list) = value {
    count = list.len();
  }

  iter_tools::repeat_n(new_placeholder, count).join(", ")
}

pub fn value<'d, T>(value: &'d T) -> FragmentValue<'d>
where
  T: ToSqlExtend + 'static,
{
  FragmentValue::value(value)
}

pub fn fragment<'d, T>(fragment: &'d T) -> FragmentValue<'d>
where
  T: FragmentLike<'d> + 'static,
{
  FragmentValue::fragment(fragment)
}

pub fn list<'d>(list: Vec<&'d dyn ToSqlExtend>) -> FragmentValue<'d> {
  FragmentValue::list(list)
}

pub trait ToSqlExtend: ToSql + std::fmt::Debug {}

impl ToSqlExtend for i32 {}
impl ToSqlExtend for &String {}
