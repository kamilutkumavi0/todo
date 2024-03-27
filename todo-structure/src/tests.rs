use crate::date::TodoDate;
use crate::todo::Todo;
use crate::todo::TodoStatus;
#[test]
fn lt_true_year(){
    let todo_date1 = TodoDate::new(1, 2, 2023).unwrap();
    let todo_date2 = TodoDate::new(1, 2, 2024).unwrap();
    assert_eq!(todo_date1<todo_date2, true);
}
#[test]
fn lt_true_month(){
    let todo_date1 = TodoDate::new(1, 2, 2023).unwrap();
    let todo_date2 = TodoDate::new(1, 3, 2023).unwrap();
    assert_eq!(todo_date1<todo_date2, true);
}
#[test]
fn lt_true_day(){
    let todo_date1 = TodoDate::new(1, 2, 2023).unwrap();
    let todo_date2 = TodoDate::new(2, 2, 2023).unwrap();
    assert_eq!(todo_date1<todo_date2, true);
}
#[test]
fn lt_false_year(){
    let todo_date1 = TodoDate::new(1, 2, 2023).unwrap();
    let todo_date2 = TodoDate::new(1, 2, 2022).unwrap();
    assert_eq!(todo_date1<todo_date2, false);
}
#[test]
fn lt_false_month(){
    let todo_date1 = TodoDate::new(1, 3, 2023).unwrap();
    let todo_date2 = TodoDate::new(1, 2, 2023).unwrap();
    assert_eq!(todo_date1<todo_date2, false);
}
#[test]
fn lt_false_day(){
    let todo_date1 = TodoDate::new(2, 2, 2023).unwrap();
    let todo_date2 = TodoDate::new(1, 2, 2023).unwrap();
    assert_eq!(todo_date1<todo_date2, false);
}
#[test]
#[should_panic]
fn todo_error(){
    let _todo_date1 = TodoDate::new(30, 2, 2023).unwrap();
}
#[test]
#[should_panic]
fn todo_error_not_leak_panic(){
    let _todo_date1 = TodoDate::new(29, 2, 2023).unwrap();
}
#[test]
#[should_panic]
fn todo_error_not_leak_panic_2(){
    let _todo_date1 = TodoDate::new(29, 2, 2100).unwrap();
}
#[test]
fn todo_leak(){
    let _todo_date1 = TodoDate::new(29, 2, 2024).unwrap();
}
#[test]
fn todo_leak_2(){
    let _todo_date1 = TodoDate::new(29, 2, 2000).unwrap();
}

#[test]
fn create_todo_only_name() {
    let new_todo = Todo::new(String::from("Kontrol"), None, None, None, None).unwrap();
    assert_eq!(new_todo,Todo {name: String::from("Kontrol"), description:None, start_date:None, finish_date:None, status:None});
}

    #[test]
    fn create_todo_with_description(){
        let new_todo = Todo::new(String::from("Kontrol"), Some(String::from("Bu yapı todo stuctunını kontrol etmekte")), None, None, None).unwrap();
        assert_eq!(new_todo,Todo {name: String::from("Kontrol"), description: Some(String::from("Bu yapı todo stuctunını kontrol etmekte")), start_date:None, finish_date:None, status:None});
        
    }
    #[test]
    fn create_todo_with_start_date(){
        let start_date = TodoDate::new(11,3,2024).unwrap();
        let new_todo = Todo::new(String::from("Kontrol"),Some(String::from("Bu yapı todo stuctunını kontrol etmekte")), Some(start_date), None, None).unwrap();
        let start_date = TodoDate::new(11,3,2024).unwrap();
        assert_eq!(new_todo, Todo {name: String::from("Kontrol"), description: Some(String::from("Bu yapı todo stuctunını kontrol etmekte")), start_date:  Some(start_date), finish_date:None, status:None});
        
    }
    #[test]
    fn create_todo_with_finish_date(){
        let finish_date = TodoDate::new(12,3,2024).unwrap();
        let new_todo = Todo::new(String::from("Kontrol"), Some(String::from("Bu yapı todo stuctunını kontrol etmekte")), None, Some(finish_date), None).unwrap();
        let finish_date = TodoDate::new(12,3,2024).unwrap();
        assert_eq!(new_todo,Todo {name: String::from("Kontrol"), description: Some(String::from("Bu yapı todo stuctunını kontrol etmekte")), start_date:None, finish_date:Some(finish_date), status:None});
        
    }
    #[test]
    fn create_todo_with_start_and_finish_date(){
        let start_date = TodoDate::new(11,3,2024).unwrap();
        let finish_date = TodoDate::new(12,3,2024).unwrap();
        let new_todo = Todo::new(String::from("Kontrol"), Some(String::from("Bu yapı todo stuctunını kontrol etmekte")), Some(start_date), Some(finish_date), None).unwrap();
        let start_date = TodoDate::new(11,3,2024).unwrap();
        let finish_date = TodoDate::new(12,3,2024).unwrap();
        assert_eq!(new_todo,Todo {name: String::from("Kontrol"), description:Some(String::from("Bu yapı todo stuctunını kontrol etmekte")), start_date:Some(start_date), finish_date: Some(finish_date), status:None});
        
    }
    #[test]
    fn create_todo_with_status(){
        let start_date = TodoDate::new(11,3,2024).unwrap();
        let finish_date = TodoDate::new(12,3,2024).unwrap();
        let status = TodoStatus::Continue;
        let new_todo = Todo::new(String::from("Kontrol"), Some(String::from("Bu yapı todo stuctunını kontrol etmekte")), Some(start_date), Some(finish_date), Some(status)).unwrap();
        let start_date = TodoDate::new(11,3,2024).unwrap();
        let finish_date = TodoDate::new(12,3,2024).unwrap();
        let status = TodoStatus::Continue;
        assert_eq!(new_todo,Todo {name: String::from("Kontrol"), description:Some(String::from("Bu yapı todo stuctunını kontrol etmekte")), start_date:Some(start_date), finish_date:Some(finish_date), status:Some(status)});
        
    }
    #[test]
    #[should_panic]
    fn no_name_error(){
        let start_date = TodoDate::new(11,3,2024).unwrap();
        let finish_date = TodoDate::new(12,3,2024).unwrap();
        let status = TodoStatus::Continue;
        let _new_todo = Todo::new(String::from(""), Some(String::from("Bu yapı todo stuctunını kontrol etmekte")), Some(start_date), Some(finish_date), Some(status)).unwrap();
        
    }
    #[test]
    #[should_panic]
    fn wrong_date_order_error(){
        let start_date = TodoDate::new(11,3,2024).unwrap();
        let finish_date = TodoDate::new(10,3,2024).unwrap();
        let status = TodoStatus::Continue;
        let _new_todo = Todo::new(String::from("Kontrol"), Some(String::from("Bu yapı todo stuctunını kontrol etmekte")), Some(start_date), Some(finish_date), Some(status)).unwrap();
        
}
