
/*
What # is used for

    # appears before attributes, like #[derive(Debug)] or #![allow(unused)], which attach metadata to items or crates.​

    In doc tests (code examples inside /// comments), # at the start of a line can hide that line from rendered docs while still running it in tests.

    Serde: for converting struct to/from JSON



    */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task{
    pub id: u64,
    pub text: String,
    pub done: bool,
}

impl Task{
    pub fn new(id :u64, text: String)-> Self{
        Self{id,text,done:false}
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TodoError{
    NotFound(u64),
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TodoList{
    pub tasks: Vec<Task>,
    pub next_id: u64,
}

impl TodoList{
    pub fn new()->Self{
        Self{
            tasks: vec![] , next_id:1
        }
    }

    pub fn add(&mut self, text:String)-> &Task{
        let id = self.next_id;
        self.next_id += 1;

        let task = Task::new(id , text);
        self.tasks.push(task);

        //return reference tp the last inserted task
        self.tasks.last().expect("just pushed a task")
    }

    pub fn list(&self) -> &[Task] {
        &self.tasks
    }

    pub fn mark_done(&mut self , id:u64) -> Result<(),TodoError> {
        for t in &mut self.tasks {
            if t.id == id {
                t.done = true;
                return Ok(());
            }
        }
        Err(TodoError::NotFound(id))
    }

    pub fn remove(&mut self , id:u64) -> Result<(),TodoError> {
        // find id of task
        let mut idx: Option<usize>=None;
        for (i,t) in self.tasks.iter().enumerate() {
            if t.id == id {
                idx = Some(i);
                break;
            }
        }

        match idx{
            Some(i)=> {
                self.tasks.remove(i);
                Ok(())
            }
            None => Err(TodoError::NotFound(id)),

        }
        
    }

    pub fn clear_done(&mut self) -> usize{
        let before = self.tasks.len();
        self.tasks.retain(|t| !t.done);
        before - self.tasks.len()
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn task_new_sets_done_false() {
        let t = Task::new(1, "buy bread".to_string());
        assert_eq!(t.id,1);
        assert_eq!(t.text, "buy bread");
        assert_eq!(t.done,false);
    }

    #[test]
    fn todo_add_creates_unique_id(){
        let mut list = TodoList::new();
        let t1 = list.add("a".to_string()).id;
        let t2 = list.add("b".to_string()).id;
        assert_eq!(t1,1);
        assert_eq!(t2,2);
        assert_eq!(list.list().len() , 2);
    }

    #[test]
    fn mark_done_sets_done_true(){
        let mut list = TodoList::new();

        let id = list.add("buy bread".to_string()).id;

        list.mark_done(id).unwrap();
        assert_eq!(list.list()[0].done , true);
    }

    #[test]
    fn mark_done_unkown_id_returns_error(){
        let mut list = TodoList::new();
        let err = list.mark_done(999).unwrap_err();
        assert_eq!(err, TodoError::NotFound(999));
    }

    #[test]
    fn remove_deletes_task(){
        let mut list = TodoList::new();
        let id1 = list.add("a".to_string()).id;
        let _id2 =list.add("b".to_string()).id;

        list.remove(id1).unwrap();
        assert_eq!(list.list().len() , 1);
        assert_eq!(list.list()[0].text , "b");
    }

    #[test]
    fn remove_unknown_id_returns_error(){
        let mut list = TodoList::new();
        let err = list.remove(999).unwrap_err();
        assert_eq!(err, TodoError::NotFound(999));
    }

    #[test]
    fn clear_done_removes_done_tasks(){
        let mut list = TodoList::new();
        let id1 = list.add("a".to_string()).id;
        let id2 = list.add("b".to_string()).id;

        list.mark_done(id1).unwrap();
        let removed = list.clear_done();
        assert_eq!(removed , 1);
        assert_eq!(list.list().len() , 1);
        assert_eq!(list.list()[0].id , id2);
    }
    
}




