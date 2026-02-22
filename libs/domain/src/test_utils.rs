use crate::clock::Clock;
use crate::id::IdGenerator;
use chrono::{DateTime, Utc};
use std::sync::Mutex;

pub struct FixedClock {
    time: DateTime<Utc>,
}

impl FixedClock {
    pub fn new(time: DateTime<Utc>) -> Self {
        Self { time }
    }
}

impl Clock for FixedClock {
    fn now(&self) -> DateTime<Utc> {
        self.time
    }
}

pub struct MockIdGenerator<T> {
    ids: Mutex<Vec<T>>,
}

impl<T> MockIdGenerator<T> {
    /// 指定された個数分、ランダムなID（Uuid::new_v4由来）を事前生成する。
    pub fn with_generated_ids(count: usize) -> Self
    where
        T: From<uuid::Uuid>,
    {
        let ids: Vec<T> = (0..count).map(|_| uuid::Uuid::new_v4().into()).collect();
        // Vecをスタックとして使うため、順番を維持したい場合はそのまま格納し pop() する。
        // （pop() は末尾から取り出すため、生成順序と一致させるなら reverse する）
        let mut reversed = ids;
        reversed.reverse();
        Self {
            ids: Mutex::new(reversed),
        }
    }

    /// 現在保持しているIDのリストを取得する（デバッグ・検証用）。
    /// pop() される順序（スタックのトップから）で返される。
    pub fn expected_ids(&self) -> Vec<T>
    where
        T: Clone,
    {
        let guard = self.ids.lock().unwrap();
        let mut ids = guard.clone();
        ids.reverse(); // 表示上、popされる順（生成順）に直す
        ids
    }
}

impl<T: Send + Sync + Clone> IdGenerator<T> for MockIdGenerator<T> {
    fn generate(&self) -> T {
        let mut guard = self.ids.lock().unwrap();
        guard.pop().expect("MockIdGenerator: No more IDs")
    }
}
