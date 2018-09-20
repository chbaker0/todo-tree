use todo_list::*;

use std::collections::HashMap;

/// An ID used to reference a `TodoList` in a `TodoListStore`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct TodoListId(pub u64);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum TodoListStoreError {
    IdNotFound,
    Other,
}

/// Represents an abstract datastore for `TodoList`s. Implementations
/// of this will be used for long-term storage of lists.
pub trait TodoListStore {
    fn create(&mut self, todo_list: &TodoList) -> Result<TodoListId, TodoListStoreError>;
    fn getone(&self, id: TodoListId) -> Result<TodoList, TodoListStoreError>;
    fn update(&mut self, id: TodoListId, todo_list: &TodoList) -> Result<(), TodoListStoreError>;
    fn delete(&mut self, id: TodoListId) -> Result<(), TodoListStoreError>;
}

/// A simple in-memory `TodoListStore` implemented using a `HashMap`.
pub struct InMemoryStore {
    cur_id: TodoListId,
    list_map: HashMap<TodoListId, TodoList>,
}

impl InMemoryStore {
    pub fn new() -> InMemoryStore {
        InMemoryStore {
            cur_id: TodoListId(0),
            list_map: HashMap::new(),
        }
    }
}

impl TodoListStore for InMemoryStore {
    fn create(&mut self, todo_list: &TodoList) -> Result<TodoListId, TodoListStoreError> {
        let id = self.cur_id;
        if id.0 == u64::max_value() {
            return Err(TodoListStoreError::Other);
        }
        self.cur_id.0 += 1;
        self.list_map.insert(id, todo_list.clone());
        Ok(id)
    }

    fn getone(&self, id: TodoListId) -> Result<TodoList, TodoListStoreError> {
        self.list_map
            .get(&id)
            .map(Clone::clone)
            .ok_or(TodoListStoreError::IdNotFound)
    }

    fn update(&mut self, id: TodoListId, todo_list: &TodoList) -> Result<(), TodoListStoreError> {
        match self.list_map.get_mut(&id) {
            Some(v) => {
                *v = todo_list.clone();
                Ok(())
            }
            None => Err(TodoListStoreError::IdNotFound),
        }
    }

    fn delete(&mut self, id: TodoListId) -> Result<(), TodoListStoreError> {
        self.list_map
            .remove(&id)
            .map(|_| ())
            .ok_or(TodoListStoreError::IdNotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_memory_store_create() {
        let mut store = InMemoryStore::new();
        let list = TodoList {
            title: "abc".to_string(),
            entries: Default::default(),
        };
        let id = store.create(&list).unwrap();
        assert_eq!(store.getone(id).unwrap().title, list.title);
    }

    #[test]
    fn in_memory_store_update() {
        let mut store = InMemoryStore::new();
        let list1 = TodoList {
            title: "abc".to_string(),
            entries: Default::default(),
        };
        let id = store.create(&list1).unwrap();
        let list2 = TodoList {
            title: "123".to_string(),
            entries: Default::default(),
        };
        store.update(id, &list2).unwrap();
        assert_eq!(store.getone(id).unwrap().title, list2.title);
    }
}
