use std::sync::Mutex;

use once_cell::sync::OnceCell;

pub struct Singleton {
    pub value: &'static str,
}

pub struct MSingleton {
  value: &'static str,
}

static INSTANCE: OnceCell<Singleton> = OnceCell::new();
static MINSTANCE: OnceCell<Mutex<MSingleton>> = OnceCell::new();

impl Singleton {
    pub fn get_instance(value: &'static str) -> &'static Singleton {
        INSTANCE.get_or_init(|| {
            Singleton { value }
        })
    }

    pub fn get_value(&self) -> &'static str {
        self.value
    }

    pub fn set_value(&mut self, value: &'static str) {
      self.value = value
    }
}

impl MSingleton {
  pub fn get_mutex_instance(value: &'static str) -> &'static Mutex<MSingleton> {
    MINSTANCE.get_or_init(|| Mutex::new(MSingleton { value }))
  }

  pub fn get_value(&self) -> &'static str {
    self.value
}
  pub fn set_value(&mut self, value: &'static str) {
    self.value = value;
  }
}