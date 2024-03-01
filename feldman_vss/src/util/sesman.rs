//! A simple in-memory key-value pool.

/// `Box<dyn Any + Send + Sync>` is used to store any type of object, \
/// thus eliminate the need of de/serialization.
static mut DB: OnceLock<DashMap<String, Box<dyn Any + Send + Sync>>> = OnceLock::new();

pub async fn send<T>(topic: &str, src: usize, dst: usize, obj: &T)
where
    T: Any + Send + Sync + Clone,
{
    let db = unsafe { DB.get_or_init(Default::default) };
    let key = format!("{}/{}->{}", topic, src, dst);
    db.insert(key, Box::new(obj.clone()));
}

pub async fn recv<T>(topic: &str, src: usize, dst: usize) -> T
where
    T: Any + Send + Sync + Clone,
{
    use tokio::time::{sleep, Duration};

    let db = unsafe { DB.get_or_init(Default::default) };
    let key = format!("{}/{}->{}", topic, src, dst);
    while !db.contains_key(&key) {
        sleep(Duration::from_millis(200)).await;
    }
    let obj = db.get(&key).unwrap();
    let obj = obj.downcast_ref::<T>().expect({
        use std::any::type_name;
        format!("Failed to interpret map entry as type {}", type_name::<T>(),).as_str()
    });
    obj.clone()
}

#[cfg(test)]
mod tests {
    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Object1(usize, usize, usize);

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Object2(String, String);

    #[tokio::test]
    async fn test_push_get() {
        let obj1 = Object1(1, 2, 3);
        let obj2 = Object2("hello".to_string(), "world".to_string());

        send("test", 1, 2, &obj1).await;
        send("test", 2, 3, &obj2).await;

        let obj1_ = recv::<Object1>("test", 1, 2).await;
        let obj2_ = recv::<Object2>("test", 2, 3).await;

        assert_eq!(obj1, obj1_);
        assert_eq!(obj2, obj2_);
    }

    use super::*;
}

/* ========== Imports ========== */
use std::{any::Any, sync::OnceLock};

use dashmap::DashMap;
