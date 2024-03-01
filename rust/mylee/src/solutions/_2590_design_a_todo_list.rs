// # [2590. Design a Todo List](https://leetcode.com/problems/design-a-todo-list)

// ## Description

//  Design a Todo List Where users can add  tasks , mark them as  complete ,
// or get a list of pending tasks. Users can also add  tags  to tasks and can filter the tasks by certain tags.

//  Implement the  TodoList  class:

// 	  TodoList()  Initializes the object.
// 	  int addTask(int userId, String taskDescription, int dueDate, List <String> tags)
// Adds a task for the user with the ID  userId  with a due date equal to  dueDate  and a list of tags attached to the task.
// The return value is the ID of the task. This ID starts at  1  and is  sequentially  increasing.
// That is, the first task's id should be  1 , the second task's id should be  2 , and so on.
// 	  List <String> getAllTasks(int userId)  Returns a list of all the tasks not marked as complete for the user with ID  userId ,
//  ordered by the due date. You should return an empty list if the user has no uncompleted tasks.
// 	  List <String> getTasksForTag(int userId, String tag)
// Returns a list of all the tasks that are not marked as complete for the user with the ID  userId  and have  tag  as one of their tags,
// ordered by their due date. Return an empty list if no such task exists.
// 	  void completeTask(int userId, int taskId)
// Marks the task with the ID  taskId  as completed only if the task exists and the user with the ID  userId  has this task, and it is uncompleted.

//  ### Example 1:

//  Input
// [ "TodoList ",  "addTask ",  "addTask ",  "getAllTasks ",  "getAllTasks ",  "addTask ",  "getTasksForTag ",  "completeTask ",  "completeTask ",  "getTasksForTag ",  "getAllTasks "]
// [[], [1,  "Task1 ", 50, []], [1,  "Task2 ", 100, [ "P1 "]], [1], [5], [1,  "Task3 ", 30, [ "P1 "]], [1,  "P1 "], [5, 1], [1, 2], [1,  "P1 "], [1]]
//  Output
// [null, 1, 2, [ "Task1 ",  "Task2 "], [], 3, [ "Task3 ",  "Task2 "], null, null, [ "Task3 "], [ "Task3 ",  "Task1 "]]

//  Explanation
// TodoList todo_list = new TodoList();
// todo_list.addTask(1,  "Task1 ", 50, []); // return 1. This adds a new task for the user with id 1.
// todo_list.addTask(1,  "Task2 ", 100, [ "P1 "]); // return 2. This adds another task for the user with id 1.
// todo_list.getAllTasks(1); // return [ "Task1 ",  "Task2 "]. User 1 has two uncompleted tasks so far.
// todo_list.getAllTasks(5); // return []. User 5 does not have any tasks so far.
// todo_list.addTask(1,  "Task3 ", 30, [ "P1 "]); // return 3. This adds another task for the user with id 1.
// todo_list.getTasksForTag(1,  "P1 "); // return [ "Task3 ",  "Task2 "].
// This returns the uncompleted tasks that have the tag  "P1 " for the user with id 1.
// todo_list.completeTask(5, 1); // This does nothing, since task 1 does not belong to user 5.
// todo_list.completeTask(1, 2); // This marks task 2 as completed.
// todo_list.getTasksForTag(1,  "P1 "); // return [ "Task3 "].
// This returns the uncompleted tasks that have the tag  "P1 " for the user with id 1.
//                                   // Notice that we did not include  "Task2 " because it is completed now.
// todo_list.getAllTasks(1); // return [ "Task3 ",  "Task1 "]. User 1 now has 2 uncompleted tasks.

//   Constraints:

// 	  1  <= userId, taskId, dueDate  <= 100
// 	  0  <= tags.length  <= 100
// 	  1  <= taskDescription.length  <= 50
// 	  1  <= tags[i].length, tag.length  <= 20
// 	 All  dueDate  values are unique.
// 	 All the strings consist of lowercase and uppercase English letters and digits.
// 	 At most  100  calls will be made for each method.

// ## Solutions

// **Approach 1: Hash Table + Sorted Set**

// We use a hash table $tasks$ to record the set of tasks for each user,
// where the key is the user ID and the value is a sorted set sorted by the deadline of the task.
// In addition, we use a variable $i$ to record the current task ID.

// When calling the `addTask` method, we add the task to the task set of the corresponding user and return the task ID.
// The time complexity of this operation is $O(\log n)$.

// When calling the `getAllTasks` method,
// we traverse the task set of the corresponding user and add the description of the unfinished task to the result list,
//  and then return the result list. The time complexity of this operation is $O(n)$.

// When calling the `getTasksForTag` method,
// we traverse the task set of the corresponding user and add the description of the unfinished task to the result list,
// and then return the result list. The time complexity of this operation is $O(n)$.

// When calling the `completeTask` method,
// we traverse the task set of the corresponding user and mark the task whose task ID is $taskId$ as completed.
// The time complexity of this operation is $(n)$.

// The space complexity is $O(n)$. Where $n$ is the number of all tasks.

use std::collections::{HashMap, HashSet};
#[allow(dead_code)]
pub struct TodoList {
    id: i32,
    tasks: HashMap<i32, HashSet<i32>>,
    task: HashMap<i32, (String, i32, HashSet<String>)>,
}

impl TodoList {
    pub fn new() -> Self {
        Self {
            id: 0,
            tasks: HashMap::new(),
            task: HashMap::new(),
        }
    }
    pub fn add_task(
        &mut self,
        user_id: i32,
        task_description: String,
        due_date: i32,
        tags: Vec<String>,
    ) -> i32 {
        self.id += 1;
        let id = self.id;
        self.task
            .insert(id, (task_description, due_date, tags.into_iter().collect()));
        self.tasks
            .entry(user_id)
            .or_insert(HashSet::new())
            .insert(id);
        id
    }
    pub fn get_all_tasks(&self, user_id: i32) -> Vec<String> {
        let mut ans = Vec::new();
        if let Some(tasks) = self.tasks.get(&user_id) {
            for task in tasks {
                if let Some(t) = self.task.get(task) {
                    ans.push((t.0.clone(), t.1));
                }
            }
        }
        ans.sort_unstable_by_key(|x| x.1);
        ans.iter().map(|x| x.0.clone()).collect()
    }
    pub fn get_tasks_for_tag(&self, user_id: i32, tag: String) -> Vec<String> {
        let mut ans = Vec::new();
        if let Some(tasks) = self.tasks.get(&user_id) {
            for task in tasks {
                if let Some(t) = self.task.get(task) {
                    if t.2.contains(&tag) {
                        ans.push((t.0.clone(), t.1));
                    }
                }
            }
        }
        ans.sort_unstable_by_key(|x| x.1);
        ans.iter().map(|x| x.0.clone()).collect()
    }
    pub fn complete_task(&mut self, user_id: i32, task_id: i32) {
        if let Some(tasks) = self.tasks.get_mut(&user_id) {
            if tasks.remove(&task_id) {
                self.task.remove(&task_id);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_todo_list_1() {
        let mut todo_list = TodoList::new();
        assert_eq!(1, todo_list.add_task(1, "Task1".to_string(), 50, vec![]));
        assert_eq!(
            2,
            todo_list.add_task(1, "Task2".to_string(), 100, vec!["P1".to_string()])
        );
        assert_eq!(
            vec!["Task1".to_string(), "Task2".to_string()],
            todo_list.get_all_tasks(1)
        );
        assert_eq!(Vec::<String>::new(), todo_list.get_all_tasks(5));
        assert_eq!(
            3,
            todo_list.add_task(1, "Task3".to_string(), 30, vec!["P1".to_string()])
        );
        assert_eq!(
            vec!["Task3".to_string(), "Task2".to_string()],
            todo_list.get_tasks_for_tag(1, "P1".to_string())
        );
        todo_list.complete_task(5, 1);
        todo_list.complete_task(1, 2);
        assert_eq!(
            vec!["Task3".to_string()],
            todo_list.get_tasks_for_tag(1, "P1".to_string())
        );
        assert_eq!(
            vec!["Task3".to_string(), "Task1".to_string()],
            todo_list.get_all_tasks(1)
        );
    }
}

// <!-- tabs:start -->

// ### **Python3**

// ```python
// from sortedcontainers import SortedList

// class TodoList:
//     def __init__(self):
//         self.i = 1
//         self.tasks = defaultdict(SortedList)

//     def addTask(
//         self, userId: int, taskDescription: str, dueDate: int, tags: List[str]
//     ) -> int:
//         taskId = self.i
//         self.i += 1
//         self.tasks[userId].add([dueDate, taskDescription, set(tags), taskId, False])
//         return taskId

//     def getAllTasks(self, userId: int) -> List[str]:
//         return [x[1] for x in self.tasks[userId] if not x[4]]

//     def getTasksForTag(self, userId: int, tag: str) -> List[str]:
//         return [x[1] for x in self.tasks[userId] if not x[4] and tag in x[2]]

//     def completeTask(self, userId: int, taskId: int) -> None:
//         for task in self.tasks[userId]:
//             if task[3] == taskId:
//                 task[4] = True
//                 break

// # Your TodoList object will be instantiated and called as such:
// # obj = TodoList()
// # param_1 = obj.addTask(userId,taskDescription,dueDate,tags)
// # param_2 = obj.getAllTasks(userId)
// # param_3 = obj.getTasksForTag(userId,tag)
// # obj.completeTask(userId,taskId)
// ```

// ### **Java**

// ```java
// class Task {
//     int taskId;
//     String taskName;
//     int dueDate;
//     Set  tags;
//     boolean finish;

//     public Task(int taskId, String taskName, int dueDate, Set  tags) {
//         this.taskId = taskId;
//         this.taskName = taskName;
//         this.dueDate = dueDate;
//         this.tags = tags;
//     }
// }

// class TodoList {
//     private int i = 1;
//     private Map<Integer, TreeSet > tasks = new HashMap<>();

//     public TodoList() {
//     }

//     public int addTask(int userId, String taskDescription, int dueDate, List  tags) {
//         Task task = new Task(i++, taskDescription, dueDate, new HashSet<>(tags));
//         tasks.computeIfAbsent(userId, k -> new TreeSet<>(Comparator.comparingInt(a -> a.dueDate)))
//             .add(task);
//         return task.taskId;
//     }

//     public List  getAllTasks(int userId) {
//         List  ans = new ArrayList<>();
//         if (tasks.containsKey(userId)) {
//             for (Task task : tasks.get(userId)) {
//                 if (!task.finish) {
//                     ans.add(task.taskName);
//                 }
//             }
//         }
//         return ans;
//     }

//     public List  getTasksForTag(int userId, String tag) {
//         List  ans = new ArrayList<>();
//         if (tasks.containsKey(userId)) {
//             for (Task task : tasks.get(userId)) {
//                 if (task.tags.contains(tag) && !task.finish) {
//                     ans.add(task.taskName);
//                 }
//             }
//         }
//         return ans;
//     }

//     public void completeTask(int userId, int taskId) {
//         if (tasks.containsKey(userId)) {
//             for (Task task : tasks.get(userId)) {
//                 if (task.taskId == taskId) {
//                     task.finish = true;
//                     break;
//                 }
//             }
//         }
//     }
// }

// /**
//  * Your TodoList object will be instantiated and called as such:
//  * TodoList obj = new TodoList();
//  * int param_1 = obj.addTask(userId,taskDescription,dueDate,tags);
//  * List  param_2 = obj.getAllTasks(userId);
//  * List  param_3 = obj.getTasksForTag(userId,tag);
//  * obj.completeTask(userId,taskId);
//  */
// ```
