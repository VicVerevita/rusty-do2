extern crate diesel;
extern crate uuid;

use crate::db::{
    models::{NewTask, Task},
    schema::tasks::dsl::*,
};
use diesel::prelude::*;
use diesel::PgConnection;
use uuid::Uuid;

pub fn create_task<'a>(
    conn: &PgConnection,
    task_title: &'a str,
    task_description: Option<&'a str>,
    status: bool,
    user: Uuid,
) -> Result<Task, diesel::result::Error> {
    let new_task = NewTask {
        title: task_title,
        description: task_description,
        finished: status,
        user_id: user,
    };

    diesel::insert_into(tasks)
        .values(&new_task)
        .get_result(conn)
}

pub fn get_user_tasks(conn: &PgConnection, user: uuid::Uuid) {
    let user_tasks = tasks
        .filter(user_id.eq(user))
        .select(Task::as_select())
        .load(conn)
        .expect("Error loading tasks!");
}

pub fn check_task(conn: &PgConnection, old_task: Task) {
    diesel::update(tasks.filter(id.eq(old_task.id)))
        .set(finished.eq(!status))
        .execute(conn)
        .expect("Failed to modify task status!");
}

pub fn update_task(
    conn: &PgConnection,
    old_task: Task,
    task_title: &str,
    task_description: Option<&str>,
    status: bool,
) {
    diesel::update(tasks.filter(id.eq(old_task.id)))
        .set((
            title.eq(task_title),
            description.eq(task_description),
            finished.eq(status),
        ))
        .execute(conn)
        .expect("Failed to update task!");
}

pub fn delete_task(conn: &PgConnection, old_task: Task) {
    diesel::delete(tasks.filter(id.eq(old_task.id)))
        .execute(conn)
        .expect("Error deleting task!");
}
