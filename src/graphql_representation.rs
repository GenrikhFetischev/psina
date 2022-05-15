use std::collections::HashSet;
use pest::iterators::Pair;
use crate::parser::Rule;
use crate::type_resolver::{GqlType, resolve_prisma_type};

#[derive(Debug)]
pub struct TypeDefinition {
  pub what_type: GqlType,
  pub is_array: bool,
  pub is_optional: bool,
}

impl From<Pair<'_, Rule>> for TypeDefinition {
  fn from(type_def: Pair<'_, Rule>) -> Self {
    let mut inner = type_def.into_inner();
    let what_type = resolve_prisma_type(inner.next().unwrap().as_str());
    let mut is_array = false;
    let mut is_optional = false;

    for modifier in inner {
      match modifier.as_rule() {
        Rule::array_type => {
          is_array = true
        }
        Rule::optional_mark => {
          is_optional = true
        }
        _ => {}
      }
    }

    TypeDefinition {
      what_type,
      is_array,
      is_optional,
    }
  }
}

impl ToString for TypeDefinition {
  fn to_string(&self) -> String {
    match (self.is_array, self.is_optional) {
      (true, true) => format!("[{}]", self.what_type.to_string()),
      (false, false) => format!("{}!", self.what_type.to_string()),
      (true, false) => format!("[{}!]!", self.what_type.to_string()),
      (false, true) => self.what_type.to_string(),
    }
  }
}


#[derive(Debug)]
pub struct Column {
  pub name: String,
  pub type_definition: TypeDefinition,
}

impl From<Pair<'_, Rule>> for Column {
  fn from(col: Pair<'_, Rule>) -> Self {
    let mut inner = col.into_inner();
    let col_name = inner.next().unwrap().as_str();
    let type_definition = inner.next();
    let type_definition = TypeDefinition::from(type_definition.unwrap());

    Column {
      name: col_name.to_owned(),
      type_definition,
    }
  }
}

impl ToString for Column {
  fn to_string(&self) -> String {
    format!("{}: {}", self.name, self.type_definition.to_string())
  }
}


#[derive(Debug)]
pub struct Model {
  pub name: String,
  pub columns: Vec<Column>,
}

impl Model {
  pub fn get_custom_types(&self) -> Vec<String> {
    let mut custom_types = HashSet::new();
    for col in &self.columns {
      if let GqlType::Custom(custom_type) = &col.type_definition.what_type {
        custom_types.insert(custom_type.to_owned());
      };
    };

    custom_types.into_iter().collect()
  }
}

impl From<Pair<'_, Rule>> for Model {
  fn from(pair: Pair<'_, Rule>) -> Self {
    let mut inner = pair.into_inner();
    let model_name = inner.next().unwrap().as_str().to_owned();
    let mut columns = Vec::new();
    for col in inner {
      if col.as_rule() == Rule::column_statement {
        columns.push(Column::from(col));
      }
    }

    Model { name: model_name, columns }
  }
}


impl ToString for Model {
  fn to_string(&self) -> String {
    let mut result = format!("type {} {{\n", self.name);
    for col in &self.columns {
      result += &format!("\t{}\n", col.to_string());
    };

    result += "}\n\n";
    result
  }
}


#[derive(Debug)]
pub struct Enum {
  pub name: String,
  pub items: Vec<String>,
}

impl From<Pair<'_, Rule>> for Enum {
  fn from(pair: Pair<'_, Rule>) -> Self {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_owned();
    let mut items = Vec::new();
    for item in inner {
      items.push(item.as_str().to_owned())
    }

    Enum {
      name,
      items,
    }
  }
}

impl ToString for Enum {
  fn to_string(&self) -> String {
    let mut result = format!("enum {} {{\n", self.name);
    for col in &self.items {
      result += &format!("\t{}\n", col);
    };
    result += "}\n\n";
    result
  }
}
