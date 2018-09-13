use todo_list::*;

use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct TodoListId(u64);

pub trait TodoListStore {
    fn create(&mut self, todo_list: &TodoList) -> Result<TodoListId, ()>;
    fn read(&self, id: TodoListId) -> Result<TodoList, ()>;
    fn update(&mut self, id: TodoListId, todo_list: &TodoList) -> Result<(), ()>;
    fn delete(&mut self, id: TodoListId) -> Result<(), ()>;
}

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
    fn create(&mut self, todo_list: &TodoList) -> Result<TodoListId, ()> {
        let id = self.cur_id;
        if id.0 == u64::max_value() {
            return Err(());
        }
        self.cur_id.0 += 1;
        self.list_map.insert(id, todo_list.clone());
        Ok(id)
    }

    fn read(&self, id: TodoListId) -> Result<TodoList, ()> {
        self.list_map.get(&id).map(Clone::clone).ok_or(())
    }

    fn update(&mut self, id: TodoListId, todo_list: &TodoList) -> Result<(), ()> {
        match self.list_map.get_mut(&id) {
            Some(v) => {
                *v = todo_list.clone();
                Ok(())
            }
            None => Err(()),
        }
    }

    fn delete(&mut self, id: TodoListId) -> Result<(), ()> {
        self.list_map.remove(&id).map(|_| ()).ok_or(())
    }
}