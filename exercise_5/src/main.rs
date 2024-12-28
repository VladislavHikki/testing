pub struct ToDoList {
    tasks: Vec<String>,
}

impl ToDoList {
    pub fn new() -> Self {
        ToDoList { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, task: String) {
        self.tasks.push(task);
    }

    pub fn remove_task(&mut self, index: usize) -> Option<String> {
        if index < self.tasks.len() {
            Some(self.tasks.remove(index))
        } else {
            None
        }
    }

    pub fn edit_task(&mut self, index: usize, new_task: String) -> Result<(), &'static str> {
        if index < self.tasks.len() {
            self.tasks[index] = new_task;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }

    pub fn get_tasks(&self) -> &Vec<String> {
        &self.tasks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_todo_list() {
        let todo_list = ToDoList::new();
        assert_eq!(todo_list.get_tasks().len(), 0);
    }

    #[test]
    fn test_add_task() {
        let mut todo_list = ToDoList::new();
        todo_list.add_task("Тест".to_string());
        assert_eq!(todo_list.get_tasks().len(), 1);
        assert_eq!(todo_list.get_tasks()[0], "Тест");
    }

    #[test]
    fn test_remove_task() {
        let mut todo_list = ToDoList::new();
        todo_list.add_task("Тест".to_string());
        todo_list.add_task("Тест2".to_string());

        let removed_task = todo_list.remove_task(0);
        assert_eq!(removed_task, Some("Тест".to_string()));
        assert_eq!(todo_list.get_tasks().len(), 1);
        assert_eq!(todo_list.get_tasks()[0], "Тест2");
    }

    #[test]
    fn test_remove_task_out_of_bounds() {
        let mut todo_list = ToDoList::new();
        todo_list.add_task("Тест".to_string());

        let removed_task = todo_list.remove_task(1);
        assert_eq!(removed_task, None);
        assert_eq!(todo_list.get_tasks().len(), 1);
        assert_eq!(todo_list.get_tasks()[0], "Тест");
    }

    #[test]
    fn test_edit_task() {
        let mut todo_list = ToDoList::new();
        todo_list.add_task("Тест".to_string());
        todo_list.add_task("Тест2".to_string());

        let result = todo_list.edit_task(1, "Тест3".to_string());
        assert!(result.is_ok());
        assert_eq!(todo_list.get_tasks()[1], "Тест3");
    }

    #[test]
    fn test_edit_task_out_of_bounds() {
        let mut todo_list = ToDoList::new();
        todo_list.add_task("Тест".to_string());

        let result = todo_list.edit_task(1, "Тест3".to_string());
        assert!(result.is_err());
        assert_eq!(todo_list.get_tasks().len(), 1);
        assert_eq!(todo_list.get_tasks()[0], "Тест");
    }
}

fn main() {}
