use kv::*;

trait KvAdapter<K, T> {
    fn set(&mut self, key: K, value: T);
    fn get(&self, key: K) -> Option<T>;
}

struct NewKv<'a, K, T>
where 
    K: Key<'a>,
    T: kv::Value
{
    store: Bucket<'a, K, T>,
}

impl<'a, K, T> NewKv<'a, K, T> 
where
    K: Key<'a>,
    T: kv::Value
{
    fn new(config_path: &str) -> Result<Self, Error> {
        let cfg = Config::new(config_path);
        let store = Store::new(cfg)?;
        let store = store.bucket::<K, T>(Some("test"))?;
        Ok(NewKv {
            store: store
        })
    }
}

impl<'a, K, T> KvAdapter<K, T> for NewKv<'a, K, T> 
where 
    K: Key<'a>,
    T: kv::Value
{
    fn get(&self, key: K) -> Option<T> {
        self.store.get(&key).unwrap()
    }

    fn set(&mut self, key: K, value: T) {
        let _ = self.store.set(&key, &value);
    }
}

fn main() -> Result<(), Error>{
    let mut store = NewKv::<&str, String>::new("./test/example1/").unwrap();
    store.set("test", "test1".to_string());
    store.set("testdummy", "dummy".to_string());
    println!("test: {}", store.get("test").unwrap());
    println!("testdummy: {}", store.get("testdummy").unwrap());
    Ok(())
}
