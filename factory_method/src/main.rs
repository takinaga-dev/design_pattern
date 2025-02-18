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
    use crate::Product;
    use crate::Factory;

    trait Sealed {}
    pub trait SelaedFactory : Sealed {
        fn create<T: Factory>(n: &mut T, v: String) -> Box<dyn Product> {
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
}
