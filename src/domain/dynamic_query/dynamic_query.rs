use crate::domain::dynamic_query::utils::get_fragment_with_values;

use super::fragment_like::FragmentLike;
use super::query_builder::QueryBuilder;
use super::utils::ToSqlExtend;

pub trait DynamicQuery {
  // fn withs(&self) -> Vec<Box<dyn FragmentLike>> {
  //   vec![]
  // }

  fn selects<'a, 'd>(&'a self) -> Vec<Box<dyn FragmentLike<'d> + 'd>> {
    vec![]
  }

  // fn froms(&self) -> Vec<Box<dyn FragmentLike>> {
  //   vec![]
  // }
  //
  // fn joins(&self) -> Vec<Box<dyn FragmentLike>> {
  //   vec![]
  // }
  //
  // fn wheres(&self) -> Vec<Box<dyn FragmentLike>> {
  //   vec![]
  // }
  //
  // fn group_bys(&self) -> Vec<Box<dyn FragmentLike>> {
  //   vec![]
  // }
  //
  // fn havings(&self) -> Vec<Box<dyn FragmentLike>> {
  //   vec![]
  // }

  fn sortables(&self) -> Vec<String> {
    vec![]
  }

  fn to_query_builder<'a, 'd>(&'a self) -> QueryBuilder<'d> {
    QueryBuilder {
      // withs: self.withs(),
      selects: self.selects(),
      // froms: self.froms(),
      // joins: self.joins(),
      // wheres: self.wheres(),
      // group_bys: self.group_bys(),
      // havings: self.havings(),
    }
  }
}

impl<'d, T: DynamicQuery> FragmentLike<'d> for T {
  fn get_parameterized_fragment(&self) -> Option<String> {
    // if let Some(pageable_request) = None {}

    self.to_query_builder().get_parameterized_fragment()
  }

  fn get_parameter_count<'a>(&'a self) -> Option<usize> {
    let builder = self.to_query_builder();
    let size = builder.get_parameter_count();
    size
  }

  fn get_parameterized_values(&self) -> Option<Vec<&'d dyn ToSqlExtend>> {
    let query_builder = self.to_query_builder();

    query_builder
      .get_parameterized_values()
      .map(|mut list| list.drain(..).map(|value| value).collect())
  }

  fn get_fragment_with_values(&self) -> Option<String> {
    get_fragment_with_values(self)
  }
}

macro_rules! fragment {
  ( $y:expr ) => {
    {
      Box::new(Fragment::new($y, vec![]))
    }
  };
  ( $y:expr, $( $x:expr ),* ) => {
    {
      Box::new(Fragment::new($y, vec![$($x,)*]))
    }
  };
}

#[cfg(test)]
mod test {
  use crate::domain::dynamic_query::dynamic_query::DynamicQuery;
  use crate::domain::dynamic_query::fragment::Fragment;
  use crate::domain::dynamic_query::fragment_like::FragmentLike;
  // use crate::domain::dynamic_query::utils::value;

  #[test]
  fn basic_dynamic_query() {
    #[derive(Debug, Deserialize)]
    struct BasicDynamicQuery {
      test: String,
    }

    impl DynamicQuery for BasicDynamicQuery {
      fn selects<'d, 'a: 'd>(&'a self) -> Vec<Box<(dyn FragmentLike<'d> + 'd)>> {
        vec![]
        // vec![Box::new(Fragment::new("? as blah3", vec![]))]
        // vec![fragment!("? as blah3", value(1i32), value(&self.test))]
      }

      // fn froms(&self) -> Vec<Box<dyn FragmentLike>> {
      //   vec![fragment!("system1"), fragment!("system2")]
      // }
    }

    let query = BasicDynamicQuery {
      test: "a".to_owned(),
    };

    println!("fragment: {:?}", query.get_parameterized_fragment(),);
    println!("values: {:?}", query.get_parameterized_values(),);
    println!(
      "fragment with values: {:?}",
      query.get_fragment_with_values(),
    );
  }
}
