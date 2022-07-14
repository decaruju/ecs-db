use std::collections::{HashMap, HashSet};
use std::ops::Add;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
enum FieldType {
    Integer(i64),
    Float(f64),
}

impl Add for FieldType {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match self {
            FieldType::Integer(self_value) => {
                match other {
                    FieldType::Integer(other_value) => FieldType::Integer(self_value + other_value),
                    FieldType::Float(other_value) => FieldType::Float(self_value as f64 + other_value),
                }
            }
            FieldType::Float(self_value) => {
                match other {
                    FieldType::Integer(other_value) => FieldType::Float(self_value + other_value as f64),
                    FieldType::Float(other_value) => FieldType::Float(self_value + other_value),
                }
            }
        }
    }
}

#[derive(Debug)]
struct Entity {
    components: HashMap<String, Component>
}

#[derive(Debug)]
#[derive(Clone)]
struct Component {
    fields: HashMap<String, FieldType>
}

struct Database {
    next_id: i64,
    entities: Vec<i64>,
    components: HashMap<String, HashMap<i64, Component>>
}

impl Database {
    fn add_entity(&mut self, component_hash: HashMap<String, Component>) -> i64 {
        let entity_id = self.next_id;
        self.next_id += 1;

        for (component_name, component) in component_hash {
            self.add_component_to_entity(entity_id, component_name, component);
        }
        self.entities.push(entity_id);

        entity_id
    }

    fn add_component_to_entity(&mut self, entity_id: i64, component_name: String, component: Component) {
        self.components.entry(component_name.clone()).or_insert(HashMap::new()).entry(entity_id).or_insert(component);
    }

    fn get_entity(&self, entity_id: i64) -> Entity {
        let mut entity = Entity{components: HashMap::new()};

        for (component_name, component_index) in &self.components {
            match component_index.get(&entity_id) {
                Some(component) => {
                    entity.components.insert(component_name.clone(), component.clone());
                },
                None => {},
            }
        }

        entity
    }

    fn get_entities_with_components(&self, component_names: Vec<String>) -> Vec<Entity> {
        let mut entity_ids: HashSet<i64> = HashSet::from_iter(self.entities.iter().cloned());

        for component_name in &component_names {
            match self.components.get(component_name) {
                Some(component_hash) => {
                    entity_ids = entity_ids.intersection(&component_hash.keys().cloned().collect()).cloned().collect();
                },
                None => {
                    entity_ids.clear();
                }
            }
        }

        let mut entities = vec![];

        for entity_id in entity_ids {
            entities.push(self.get_entity(entity_id));
        }

        entities
    }

    fn update_component_field(&mut self, entity_id: i64, component_name: &String, field: &String, value: FieldType) -> bool {
        match self.components.get_mut(component_name) {
            Some(component_hash) => {
                match component_hash.get_mut(&entity_id) {
                    Some(component) => {
                        component.fields.insert(field.clone(), value);
                        true
                    },
                    _ => false,
                }
            },
            _ => false,
        }
    }

    fn get_component_field_value(&self, entity_id: i64, component_name: &String, field: &String) -> Option<FieldType> {
        match self.components.get(component_name) {
            Some(component_hash) => {
                match component_hash.get(&entity_id) {
                    Some(component) => {
                        match component.fields.get(field) {
                            Some(field) => {
                                Some(*field)
                            },
                            _ => None
                        }
                    }
                    _ => None,
                }
            },
            _ => None,
        }
    }

    fn increment_component_field(&mut self, entity_id: i64, component_name: &String, field: &String, value: FieldType) -> bool {
        if let Some(current_value) = self.get_component_field_value(entity_id, &component_name, &field) {
            self.update_component_field(entity_id, &component_name, &field, current_value + value)
        } else {
            false
        }
    }
}

fn main() {
    let mut database = Database {
        next_id: 1,
        entities: vec![],
        components: HashMap::new(),
    };

    let mut components = HashMap::new();
    let mut component_hash = HashMap::new();
    component_hash.insert("x".to_string(), FieldType::Float(0.0));
    component_hash.insert("y".to_string(), FieldType::Float(0.0));
    components.insert("position".to_string(), Component{fields: component_hash});

    let entity_id = database.add_entity(components);
    database.add_entity(HashMap::new());

    println!("{:?}", database.get_entity(entity_id));
    println!("{:?}", database.get_entities_with_components(vec!["position".to_string()]));
    database.update_component_field(entity_id, &"position".to_string(), &"x".to_string(), FieldType::Float(1.0));
    println!("{:?}", database.get_entities_with_components(vec!["position".to_string()]));
    database.increment_component_field(entity_id, &"position".to_string(), &"x".to_string(), FieldType::Float(1.0));
    println!("{:?}", database.get_entities_with_components(vec!["position".to_string()]));
    println!("{:?}", database.get_entities_with_components(vec![]));
}
