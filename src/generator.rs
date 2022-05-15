use crate::{Enum, Model};
use std::collections::HashSet;

pub fn generate_gql_schema(models: &[Model], enums: &[Enum]) -> String {
    let mut gql_spec_content = "".to_string();

    for custom_type in get_custom_types(models, enums) {
        gql_spec_content += &format!("scalar {} \n", custom_type);
    }

    for enumeration in enums {
        gql_spec_content += &enumeration.to_string()
    }

    for model in models {
        gql_spec_content += &model.to_string();
    }

    gql_spec_content
}

fn get_custom_types(models: &[Model], enums: &[Enum]) -> Vec<String> {
    let mut custom_types = HashSet::new();
    let mut type_names = HashSet::new();

    for enumeration in enums {
        type_names.insert(enumeration.name.to_owned());
    }

    for model in models {
        type_names.insert(model.name.to_owned());
        custom_types.remove(&model.name);

        let model_custom_types = model.get_custom_types();

        for custom_type in model_custom_types {
            if !type_names.contains(&custom_type) {
                custom_types.insert(custom_type);
            } else {
                custom_types.remove(&custom_type);
            }
        }
    }

    custom_types.into_iter().collect()
}
