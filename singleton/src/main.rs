mod singleton;

fn main() {
    let singleton = singleton::Singleton::get_instance("hello world");
    let singleton2 = singleton::Singleton::get_instance("hello world2");
    println!("{}", singleton.get_value());
    println!("{}", singleton2.get_value());
    // mutable
    let msingleton = singleton::MSingleton::get_mutex_instance("heelo");
    let mut v = msingleton.lock().unwrap();
    println!("{}", v.get_value());
    v.set_value("sample");
    //明示的にunlock
    std::mem::drop(v);

    println!("{}", msingleton.lock().unwrap().get_value());
}
