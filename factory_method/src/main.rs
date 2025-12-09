use internal::{ImplSealedFactory, SelaedFactory};

trait Factory {
    fn createProduct(&mut self, value: String) -> Box<dyn Product>;
}
trait Product {
    fn test(&mut self);
}

struct ImplProduct {
    value: String
}

impl ImplProduct {
    fn new(v: String) -> Self {
        ImplProduct {
            value: v
        }
    }
}
// 追加実装
struct ImplNewProduct {
    version: i32
}

impl ImplNewProduct {
    fn new(v: i32) -> Self {
        ImplNewProduct {
            version: v
        }
    }
}

impl Product for ImplNewProduct {
    fn test(&mut self) {
        println!("version: {}", self.version);
    }
}

#[derive(Debug)]
struct newImplFactory {}
impl newImplFactory {
    fn new() -> Self {
        newImplFactory {}
    }
}

impl Factory for newImplFactory {
    fn createProduct(&mut self, value: String) -> Box<dyn Product> {
        Box::new(ImplNewProduct::new(value.parse().unwrap()))
    }
}
#[derive(Debug)]
struct ImplFactory {}
impl ImplFactory {
    fn new() -> Self {
        ImplFactory {}
    }
}

// ImplProductを生成するロジックはImplFactoryで実装して
// それをImplSealedFactoryにわたす。
// どんなFactoryを渡してもそこで定義したcreateProduct関数を実行する
// Productの実装がImplProductしかないため、有用性が伝わりにくいかもしれない。
impl Factory for ImplFactory {
    fn createProduct(&mut self, value: String ) -> Box<dyn Product> {
        Box::new(ImplProduct::new(value))
    }
}
impl Product for ImplProduct {
    fn test(&mut self) {
        println!("output for ImplProduct: {}", self.value);
    }
}
mod internal {
    use std::fmt::Debug;

    use crate::Product;
    use crate::Factory;

    trait Sealed {}
    pub trait SelaedFactory : Sealed {
        fn create<T: Factory + Debug>(n: &mut T, v: String) -> Box<dyn Product> {
            println!("Befor create {:?} ", n);
            n.createProduct(v)
        }
    }
    pub struct ImplSealedFactory {}
    impl Sealed for ImplSealedFactory {}
    impl SelaedFactory for ImplSealedFactory {}

}
fn main() {
    let mut ImplF = ImplFactory::new();
    let mut product = ImplSealedFactory::create(&mut ImplF, "Hello World".to_string());
    let p = product.as_mut();
    p.test();

    let mut newF = newImplFactory::new();;
    let mut newP = ImplSealedFactory::create(&mut newF, "2".to_string());
    newP.test();
}
