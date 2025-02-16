use internal::{FinalHook};

// ユーザーが変更可能なtrait
trait template {
    fn start(&self);
    fn exec(&self);
    fn end(&self);
}

mod internal {
    trait Sealed {}
    // Hooks traitを実装する型はSealedとtemplateが実装されていることを保証する
    pub trait FinalHook: Sealed {
        fn play<T: super::template>(t: &T) {
            t.start();
            t.exec();
            t.end();
        }
    }
    pub struct ImplFinalHook {}
    impl Sealed for ImplFinalHook{}
    impl FinalHook for ImplFinalHook{}

}
struct ImplTemple {
    value: String
}

impl ImplTemple {
    fn new(v: String) -> Self {
        ImplTemple {
            value: v
        }
    }
}

impl template for ImplTemple {
    fn start(&self) {
        println!("before exec");
    }

    fn exec(&self) {
        println!("{}", self.value);
    }

    fn end(&self) {
        println!("after exec");
    }
}

fn main() {
    let obj = ImplTemple::new("hello".to_string());
    internal::ImplFinalHook::play(&obj);
}
