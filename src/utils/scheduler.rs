use std::collections::HashMap;

#[derive(Default)]
pub struct ServerScheduler {
    tasks: HashMap<u64, ScheduledTask>,
    next_id: u64,
}

pub struct ScheduledTask {
    cooldown: u64,
    delay: u64,
    func: fn(),
    repeat: bool,
}

impl ServerScheduler {
    pub fn run_task(&mut self, func: fn(), delay: u64, repeat: bool) -> u64 {
        let task = ScheduledTask {
            func,
            cooldown: delay,
            delay,
            repeat,
        };

        self.tasks.insert(self.next_id, task);
        self.next_id += 1;
        self.next_id
    }

    pub fn cancel_task(&mut self, id: u64) {
        self.tasks.remove(&id);
    }

    pub fn tick(&mut self) {
        let mut completed = vec![];
        for (id, task) in self.tasks.iter_mut() {
            task.cooldown -= 1;
            if task.cooldown == 0 {
                if task.repeat {
                    task.cooldown = task.delay;
                } else {
                    completed.push(*id);
                }
                (task.func)()
            }
        }
        for id in completed {
            self.tasks.remove(&id);
        }
    }
}
