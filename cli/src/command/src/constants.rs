use serde::Serialize;

// Component data structure for Handlebars
#[derive(Serialize)]
pub struct ComponentData {
    pub name: String,
}

// Handlebars template for the component
pub const COMPONENT_TEMPLATE: &str = r#"import React from 'react'
import { View, Text } from 'react-native'

export default function {{name}}() {
  return (
    <View style={styles.container}>
      <Text style={styles.text}>{{name}}</Text>
    </View>
  )
}
"#;
