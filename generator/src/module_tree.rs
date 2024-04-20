use std::collections::BTreeMap;
use std::sync::Arc;

use crate::module_builder::{ModuleBuilder, ResolvedType};

///! Builds a tree from a flat module representation
#[derive(Default)]
pub struct Item {
    pub children: BTreeMap<String, Item>,
    pub objects: Vec<Arc<ResolvedType>>,
}

pub struct ModuleTree {
    pub root: Item,
}

fn insert(item: &mut Item, path: &[String], ty: &Arc<ResolvedType>) {
    if path.len() == 0 {
        item.objects.push(ty.clone());
    } else {
        let item = item.children.entry(path[0].clone()).or_insert_with(Item::default);
        insert(item, &path[1..], ty);
    }
}

impl ModuleTree {
    pub fn build(module_builder: &ModuleBuilder) -> Self {
        // Root item
        let mut root = Item {
            children: BTreeMap::new(),
            objects: Vec::new(),
        };

        for ty in module_builder.types.values() {
            insert(&mut root, &ty.module_path, ty);
        }

        ModuleTree {
            root
        }
    }
}
