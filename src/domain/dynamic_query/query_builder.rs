use super::fragment_like::FragmentLike;
use super::utils::ToSqlExtend;

pub struct QueryBuilder<'d> {
  // pub withs: Vec<Box<dyn FragmentLike>>,
  pub selects: Vec<Box<dyn FragmentLike<'d>>>,
  // pub froms: Vec<Box<dyn FragmentLike>>,
  // pub joins: Vec<Box<dyn FragmentLike>>,
  // pub wheres: Vec<Box<dyn FragmentLike>>,
  // pub group_bys: Vec<Box<dyn FragmentLike>>,
  // pub havings: Vec<Box<dyn FragmentLike>>,
}

impl<'d> QueryBuilder<'d> {
  pub fn generate_query(&self) -> String {
    vec![
      // Self::join_fragments_prefix(
      //   ",\n",
      //   &self.withs.iter().map(|frag| &**frag).collect(),
      //   "WITH",
      // ),
      Self::join_fragments_prefix(",\n", &self.selects, "SELECT"),
      // Self::join_fragments_prefix(
      //   ",\n",
      //   &self.froms.iter().map(|frag| &**frag).collect(),
      //   "FROM",
      // ),
      // Self::join_fragments("\n", &self.joins.iter().map(|frag| &**frag).collect()),
      // Self::join_fragments_prefix(
      //   " AND",
      //   &self.wheres.iter().map(|frag| &**frag).collect(),
      //   "WHERE",
      // ),
      // Self::join_fragments_prefix(
      //   ",\n",
      //   &self.group_bys.iter().map(|frag| &**frag).collect(),
      //   "GROUP BY",
      // ),
      // Self::join_fragments_prefix(
      //   "\n",
      //   &self.havings.iter().map(|frag| &**frag).collect(),
      //   "HAVING",
      // ),
    ]
    .drain(..)
    .filter_map(|thing| thing)
    .collect::<Vec<_>>()
    .join("\n")
  }

  fn join_fragments<'a>(
    join: &str,
    fragments: &'a Vec<Box<dyn FragmentLike<'d>>>,
  ) -> Option<String> {
    Some(fragments).filter(|list| !list.is_empty()).map(|list| {
      list
        .iter()
        .map(|item| item.get_parameterized_fragment())
        .filter_map(|thing| thing)
        .collect::<Vec<_>>()
        .join(join)
    })
  }

  fn join_fragments_prefix<'a>(
    join: &str,
    fragments: &'a Vec<Box<dyn FragmentLike<'d>>>,
    prefix: &str,
  ) -> Option<String> {
    Self::join_fragments(join, fragments).map(|fragment| format!("{} {}", prefix, &fragment))
  }
}

impl<'d> FragmentLike<'d> for QueryBuilder<'d> {
  fn get_parameterized_fragment(&self) -> Option<String> {
    Some(Self::generate_query(self))
  }

  fn get_parameter_count(&self) -> Option<usize> {
    self
      .get_parameterized_values()
      .map(|list| list.len())
      .or(Some(0))
  }

  fn get_parameterized_values(&self) -> Option<Vec<&'d dyn ToSqlExtend>> {
    Some(
      vec![
        // &self.withs,
        &self.selects,
        // &self.froms,
        // &self.joins,
        // &self.wheres,
        // &self.group_bys,
        // &self.havings,
      ]
      .iter()
      .flat_map(|value| {
        value
          .iter()
          .filter_map(|value| value.get_parameterized_values())
          .flatten()
      })
      .collect(),
    )
  }

  fn get_fragment_with_values(&self) -> Option<String> {
    let values = self.get_parameterized_values().unwrap_or(vec![]);
    let new_fragment = self.get_parameterized_fragment();

    new_fragment.map(|mut fragment| {
      for value in values {
        fragment = fragment.replacen("?", &format!("'{:?}'", &value), 1);
      }

      fragment.replace("\\:\\:", "::")
    })
  }
}
