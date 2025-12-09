use std::{cell::RefCell, rc::Rc, vec};

/*
Directory/Fileの基底trait(Interface)
 */
trait Entry {
    fn get_name(&self) -> &str;
    fn get_size(&self) -> u32;
    fn print_list(&self, prefix: &str);
    // entryは共有所有権
    // dynはEntryを実装しているstruct
    // RefCellは可変参照
    // つまりentryはEntry traitを実装しており関数内で値が変更される可能性があり、後続の処理で再利用される可能性がある変数である
    fn add(&mut self, entry: Rc<RefCell<dyn Entry>>) -> Result<(), String>;
    fn get_parentpath(&self) -> String;
}

struct File {
    // rootのparentdirは存在しないためOption
    // この型パズルが独力で設定できる気がしない
    parentdir: Option<Rc<RefCell<Directory>>>,
    name: String,
    size: u32,
}

struct Directory {
    parentdir: Option<Rc<RefCell<Directory>>>,
    name: String,
    entries: Vec<Rc<RefCell<dyn Entry>>>,
}

impl Entry for Directory {
    // &strはStringの部分参照
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_size(&self) -> u32 {
        let mut size = 0;
        for entry in self.entries.iter().clone() {
            size += entry.borrow().get_size();
        }

        size
    }

    fn print_list(&self, prefix: &str) {
        for entry in self.entries.iter().clone() {
            entry.borrow().print_list(&format!("{}/{}",prefix, self.name));
        }
    }

    fn add(&mut self, entry: Rc<RefCell<dyn Entry>>) -> Result<(), String> {
        self.entries.push(entry);

        Ok(())
    }

    fn get_parentpath(&self) -> String {
        if self.parentdir.is_none() {
            return format!("/{}", self.name);
        }
        format!("{}/{}", self.parentdir.as_ref().unwrap().borrow().get_parentpath(),self.name)
    }
}

impl Directory {
    fn new(name: String, parentdir: Option<Rc<RefCell<Directory>>>) -> Self {
        Directory {name, entries: Vec::new(), parentdir}
    }
}
impl Entry for File {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_size(&self) -> u32 {
        self.size
    }

    fn print_list(&self, prefix: &str) {
        println!("{}/{}", prefix, self.get_name());
    }

    fn add(&mut self, _entry: Rc<RefCell<dyn Entry>>) -> Result<(), String> {
        // ファイルにはaddできないので何もしない
        Err("FileTreatmentException: ファイルにはエントリを追加できません".to_string())
    }

    fn get_parentpath(&self) -> String {
        if self.parentdir.is_none() {
            return format!("/{}", self.name);
        }
        format!("{}/{}", self.parentdir.as_ref().unwrap().borrow().get_parentpath(),self.name)
    }
}

impl File {
    fn new(name: String, size: u32, parentdir: Option<Rc<RefCell<Directory>>>) -> Self {
        File { name, size, parentdir}
    }
}

fn main() {
    println!("Making root entries...");
    let rootdir = Rc::new(RefCell::new(Directory::new("root".to_string(), None)));
    let bindir = Rc::new(RefCell::new(Directory::new("bin".to_string(), Some(rootdir.clone()))));
    let tmpdir = Rc::new(RefCell::new(Directory::new("tmp".to_string(), Some(rootdir.clone()))));
    let usrdir = Rc::new(RefCell::new(Directory::new("usr".to_string(), Some(rootdir.clone()))));
    let _ = rootdir.borrow_mut().add(bindir.clone());
    let _ = rootdir.borrow_mut().add(tmpdir);
    let _ = rootdir.borrow_mut().add(usrdir.clone());
    let _ = bindir.borrow_mut().add(Rc::new(RefCell::new(File::new("vi".to_string(), 10000, Some(bindir.clone())))));
    let _ = bindir.borrow_mut().add(Rc::new(RefCell::new(File::new("latex".to_string(), 20000, Some(bindir.clone())))));

    rootdir.borrow_mut().print_list("");

    println!("Makeing user entries");
    let yuki = Rc::new(RefCell::new(Directory::new("yuki".to_string(), Some(usrdir.clone()))));
    let hanako = Rc::new(RefCell::new(Directory::new("hanako".to_string(), Some(usrdir.clone()))));
    let tomura = Rc::new(RefCell::new(Directory::new("tomura".to_string(), Some(usrdir.clone()))));

    let _ = usrdir.borrow_mut().add(yuki.clone());
    let _ = usrdir.borrow_mut().add(hanako.clone());
    let _ = usrdir.borrow_mut().add(tomura.clone());

    let f = Rc::new(RefCell::new(File::new("diary.html".to_string(), 100, Some(yuki.clone()))));
    let _ = yuki.borrow_mut().add(f.clone());
    let _ = yuki.borrow_mut().add(Rc::new(RefCell::new(File::new("Composite.java".to_string(), 200, Some(yuki.clone())))));
    let _ = hanako.borrow_mut().add(Rc::new(RefCell::new(File::new("memo.txt".to_string(), 300, Some(hanako.clone())))));
    let _ = tomura.borrow_mut().add(Rc::new(RefCell::new(File::new("game.doc".to_string(), 400, Some(tomura.clone())))));
    let _ = tomura.borrow_mut().add(Rc::new(RefCell::new(File::new("junk.mail".to_string(), 500, Some(tomura.clone())))));

    rootdir.borrow().print_list("");
    println!("all file size is {} bytes", rootdir.borrow().get_size());
    println!("full path from leaf file: {}", f.borrow().get_parentpath());
}
