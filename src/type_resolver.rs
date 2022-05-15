#[derive(Debug)]
pub enum GqlType {
  ID,
  String,
  Boolean,
  Int,
  Float,
  Custom(String),
}

impl ToString for GqlType {
  fn to_string(&self) -> String {
    match self {
      GqlType::ID => "ID".to_owned(),
      GqlType::String => "String".to_owned(),
      GqlType::Boolean => "Boolean".to_owned(),
      GqlType::Int => "Int".to_owned(),
      GqlType::Float => "Float".to_owned(),
      GqlType::Custom(custom_type) => custom_type.to_owned()
    }
  }
}


pub fn resolve_prisma_type(type_string: &str) -> GqlType {
  match type_string {
    "ID" => GqlType::ID,
    "String" => GqlType::String,
    "Boolean" => GqlType::Boolean,
    "Int" => GqlType::Int,
    "Float" => GqlType::Float,
    _ => GqlType::Custom(type_string.to_owned())
  }
}
