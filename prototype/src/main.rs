use std::collections::HashMap;

// 新しいトレイトを作成してObjectSafeにする
trait Product {
    fn tukau(&mut self, s: String);
    fn create_clone(&self) -> Box<dyn Product>;
}


#[derive(Clone)]
struct UnderlinePen {
    ulchar: String
}

#[derive(Clone)]
struct MessageBox {
    decochar: String
}

// 新しいCloneableProductの実装を追加
impl Product for UnderlinePen {
    fn tukau(&mut self, s: String) {
        println!("***** {} *******", s);
    }
    
    fn create_clone(&self) -> Box<dyn Product> {
        Box::new(self.clone())
    }
}

impl Product for MessageBox {
    fn tukau(&mut self, s: String) {
        for _ in 0..4 {
            print!("{}", self.decochar);
        }
        println!("");
        println!("{}", s);
        for _ in 0..4 {
            print!("{}", self.decochar);
        }
        println!("");
    }
    
    fn create_clone(&self) -> Box<dyn Product> {
        Box::new(self.clone())
    }
}

// 既存の実装
impl UnderlinePen {
    fn new(s: String) -> Self {
        Self {
            ulchar: s
        }
    }
}

impl MessageBox {
    fn new(s: String) -> Self {
        Self {
            decochar: s
        }
    }
}

// Managerをトレイトオブジェクトで実装
struct Manager {
    showcase: HashMap<String, Box<dyn Product>>
}

impl Manager {
    fn register(&mut self, name: String, proto: Box<dyn Product>) {
        self.showcase.insert(name, proto);
    }

    fn create(&self, name: &str) -> Option<Box<dyn Product>> {
        self.showcase.get(name).map(|product| product.create_clone())
    }

    fn new() -> Self {
        Self {
            showcase: HashMap::new()
        }
    }
}

// こっからはAI
// 実用的な例：データベース接続設定
#[derive(Clone)]
struct DatabaseConnection {
    host: String,
    port: u16,
    username: String,
    password: String,
    connection_pool: Vec<String>, // 実際には複雑な接続プールオブジェクト
    is_ssl_enabled: bool,
    timeout_ms: u32,
}

impl Product for DatabaseConnection {
    fn tukau(&mut self, query: String) {
        println!("Executing query on {}:{} - {}", self.host, self.port, query);
        // 実際のクエリ実行ロジック
    }
    
    fn create_clone(&self) -> Box<dyn Product> {
        Box::new(self.clone())
    }
}

impl DatabaseConnection {
    // 複雑な初期化プロセス（時間がかかる）
    fn new_with_complex_setup(host: String, port: u16) -> Self {
        println!("Setting up complex database connection...");
        // 実際にはここで：
        // - SSL証明書の検証
        // - 接続プールの初期化
        // - 認証情報の暗号化
        // - ネットワーク接続のテスト
        
        Self {
            host,
            port,
            username: "admin".to_string(),
            password: "encrypted_password".to_string(),
            connection_pool: vec!["conn1".to_string(), "conn2".to_string()],
            is_ssl_enabled: true,
            timeout_ms: 5000,
        }
    }
}

// ゲーム開発の例：キャラクタープリセット
#[derive(Clone)]
struct GameCharacter {
    name: String,
    health: u32,
    mana: u32,
    skills: Vec<String>,
    equipment: Vec<String>,
    stats: CharacterStats,
}

#[derive(Clone)]
struct CharacterStats {
    strength: u32,
    agility: u32,
    intelligence: u32,
    charisma: u32,
}

impl Product for GameCharacter {
    fn tukau(&mut self, action: String) {
        println!("{} performs: {}", self.name, action);
        // スキルやステータスに応じた行動ロジック
    }
    
    fn create_clone(&self) -> Box<dyn Product> {
        Box::new(self.clone())
    }
}

impl GameCharacter {
    // プリセットキャラクターの定義
    fn warrior_preset() -> Self {
        Self {
            name: "Warrior Template".to_string(),
            health: 100,
            mana: 20,
            skills: vec!["Sword Strike".to_string(), "Shield Bash".to_string()],
            equipment: vec!["Iron Sword".to_string(), "Iron Shield".to_string()],
            stats: CharacterStats {
                strength: 15,
                agility: 8,
                intelligence: 5,
                charisma: 7,
            },
        }
    }
    
    fn mage_preset() -> Self {
        Self {
            name: "Mage Template".to_string(),
            health: 60,
            mana: 100,
            skills: vec!["Fireball".to_string(), "Ice Bolt".to_string(), "Heal".to_string()],
            equipment: vec!["Magic Staff".to_string(), "Robe".to_string()],
            stats: CharacterStats {
                strength: 5,
                agility: 6,
                intelligence: 18,
                charisma: 12,
            },
        }
    }
    
    fn customize(&mut self, name: String) {
        self.name = name;
        // 追加のカスタマイズロジック
    }
}

fn main() {
   let mut manager = Manager::new();
   
   // AI
   // データベース接続のプロトタイプを作成（初期化は一度だけ）
   let db_prototype = DatabaseConnection::new_with_complex_setup(
       "production-db.example.com".to_string(), 
       5432
   );
   manager.register("prod_db".to_string(), Box::new(db_prototype));
   
   // ゲームキャラクターのプリセットを登録
   manager.register("warrior".to_string(), Box::new(GameCharacter::warrior_preset()));
   manager.register("mage".to_string(), Box::new(GameCharacter::mage_preset()));
   
   // 複数のデータベース接続を素早く作成（複雑な初期化は不要）
   for i in 1..=3 {
       if let Some(mut db_conn) = manager.create("prod_db") {
           db_conn.tukau(format!("SELECT * FROM users WHERE id = {}", i));
       }
   }
   
   // ゲームでプレイヤーキャラクターを作成
   if let Some(mut player1) = manager.create("warrior") {
       player1.tukau("Attack with sword".to_string());
   }
   
   if let Some(mut player2) = manager.create("mage") {
       player2.tukau("Cast fireball".to_string());
   }
   // AI end

   let pen1 = UnderlinePen::new("*".to_string());
   manager.register("underline".to_string(), Box::new(pen1));
   let pen2: MessageBox = MessageBox::new("*".to_string());
   manager.register("p2".to_string(), Box::new(pen2));

   if let Some(mut u) = manager.create("underline") {
    u.tukau("Hello World".to_string());
   }

   if let Some(mut m) = manager.create("p2") {
    m.tukau("Hello Message".to_string());
   }
}
