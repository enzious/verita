use super::utils::ToSqlExtend;

pub trait FragmentLike<'d> {
  fn get_parameterized_fragment(&self) -> Option<String>;

  fn get_parameter_count(&self) -> Option<usize>;

  fn get_parameterized_values<'a>(&'a self) -> Option<Vec<&'d dyn ToSqlExtend>>;

  fn get_fragment_with_values(&self) -> Option<String>;
}

fn values<'a, T>(mut values: Vec<T>) -> Vec<FragmentValue<'a>>
where
  T: Into<FragmentValue<'a>>,
{
  values.drain(..).map(|item| item.into()).collect()
}

pub enum FragmentValue<'d> {
  Value(&'d dyn ToSqlExtend),
  Fragment(&'d dyn FragmentLike<'d>),
  List(Vec<&'d dyn ToSqlExtend>),
}

impl<'d> FragmentValue<'d> {
  pub fn value<T>(value: &'d T) -> FragmentValue<'d>
  where
    T: ToSqlExtend + 'static,
  {
    Self::Value(value)
  }

  pub fn fragment<T>(fragment: &'d T) -> FragmentValue<'d>
  where
    T: FragmentLike<'d> + 'static,
  {
    Self::Fragment(fragment)
  }

  pub fn list(list: Vec<&'d dyn ToSqlExtend>) -> FragmentValue<'d> {
    Self::List(list)
  }
}
