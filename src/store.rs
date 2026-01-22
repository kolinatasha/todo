use crate::model::TodoList;
use std::fs;
use std::path::Path;

pub fn load(path: &Path) -> Result<TodoList,String>{
    if !path.exists(){
        return Ok(TodoList::new());
    }

    let data = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let list: TodoList = serde_json::from_str(&data).map_err(|e| e.to_string())?;
    Ok(list)
}


pub fn save(path: &Path, list: &TodoList)-> Result<(),String>{
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let data = serde_json::to_string_pretty(list).map_err(|e| e.to_string())?;
    fs::write(path,data).map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg(test)]
mod tests{
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn save_then_load_roudtrip(){
        // create todo list
        let mut list = TodoList::new();
        list.add("a".to_string());
        let _id2 = list.add("b".to_string()).id;
        list.mark_done(_id2).unwrap();

        // temp file path
        let mut path = std::env::temp_dir();
        path.push("todo_test_todos.json");

        // save and load
        save(&path, &list).unwrap();
        let loaded = load(&path).unwrap();

        // verify
        assert_eq!(loaded.tasks.len(),2);
        assert_eq!(loaded.tasks[1].done , true);

        //cleanup
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn load_missing_file_returns_empty_list(){
        let mut path = PathBuf::from(std::env::temp_dir());
        path.push("todo_missing_file_hopefully.json");

        //ensure its missing
        let _ = std::fs::remove_file(&path);

        //verify
        let loaded = load(&path).unwrap();
        assert_eq!(loaded.tasks.len(),0);
        assert_eq!(loaded.next_id , 1);
    }
}