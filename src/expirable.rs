use std::time::SystemTime;

#[derive(Debug)]
pub struct Expirable<T> {
    data: T,
    created_at: SystemTime,
}

impl<T> Expirable<T> {
    pub fn now(data: T) -> Self {
        Self {
            data,
            created_at: SystemTime::now(),
        }
    }

    pub fn created_at(&self) -> SystemTime {
        self.created_at
    }

    pub fn with_timestamp(data: T, created_at: SystemTime) -> Self {
        Self { data, created_at }
    }

    pub fn expired(&self, new_time: SystemTime) -> bool {
        self.created_at < new_time
    }

    pub fn data(&self) -> &'_ T {
        &self.data
    }
}
