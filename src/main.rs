fn main() {
    // Styling constants
    const TITLE_STYLE: &str = "\x1b[1;97;103m"; // Bold White on Orange background
    const HEADER_STYLE: &str = "\x1b[1;93m"; // Bold Bright Yellow (acts as orange)
    const SUB_HEADER_STYLE: &str = "\x1b[1;97m"; // Bold White
    const COMMENT_STYLE: &str = "\x1b[32m"; // Green
    const URL_STYLE: &str = "\x1b[90m"; // Dim Gray
    const RESET: &str = "\x1b[0m";

    // Main Title and Separator
    println!(
        "{}=========================================================={}",
        HEADER_STYLE, RESET
    );
    println!(
        "{}RUST CHEATSHEET{}           original by {}Francesco Ciulla{}",
        TITLE_STYLE, RESET, HEADER_STYLE, RESET
    );
    println!(
        "{}=========================================================={}",
        HEADER_STYLE, RESET
    );
    println!("");

    // --- BASIC SYNTAX ---
    println!("{}>>> Basic Syntax <<<{}\n", HEADER_STYLE, RESET);

    // Variables
    println!("{}   Variables{} ", SUB_HEADER_STYLE, RESET);
    println!("{}    // immutable by default{}", COMMENT_STYLE, RESET);
    println!("    let x = 5;");
    println!("{}    // mutable variable{}", COMMENT_STYLE, RESET);
    println!("    let mut y = 10;\n");

    // Constants
    println!("{}   Constants{} ", SUB_HEADER_STYLE, RESET);
    println!("    const MAX_POINTS: u32 = 100_000;\n");

    // Basic Data Types
    println!("{}   Basic Data Types{} ", SUB_HEADER_STYLE, RESET);
    println!("    let a: i32 = 10;");
    println!("    let b: f64 = 3.14;");
    println!("    let c: bool = true;");
    println!("    let d: char = 'z';");
    println!("    let e: &str = \"Hello\";\n");

    // Control Flow
    println!("{}   Control Flow{} ", SUB_HEADER_STYLE, RESET);
    println!("    if condition {{");
    println!("        // code");
    println!("    }} else if another_condition {{");
    println!("        // code");
    println!("    }} else {{");
    println!("        // code");
    println!("    }}\n");

    println!("    for i in 0..5 {{");
    println!("        // code");
    println!("    }}\n");

    println!("    while condition {{");
    println!("        // code");
    println!("    }}\n");

    println!("    loop {{");
    println!("{}        // infinite loop,{}", COMMENT_STYLE, RESET);
    println!("{}        // break to exit{}", COMMENT_STYLE, RESET);
    println!("    }}\n");

    // --- FUNCTIONS ---
    println!(
        "{}----------------------------------------------------------{}",
        URL_STYLE, RESET
    );
    println!("{}>>> Functions <<<{}\n", HEADER_STYLE, RESET);

    // Function Definition
    println!("{}   Function Definition{} ", SUB_HEADER_STYLE, RESET);
    println!("    fn add(x: i32, y: i32) -> i32 {{");
    println!("        x + y");
    println!("    }}\n");

    // Closures
    println!("{}   Closures{} ", SUB_HEADER_STYLE, RESET);
    println!("    let add = |x, y| x + y;\n");

    // --- OWNERSHIP & BORROWING ---
    println!(
        "{}----------------------------------------------------------{}",
        URL_STYLE, RESET
    );
    println!("{}>>> Ownership & Borrowing <<<{}\n", HEADER_STYLE, RESET);

    // Ownership Rules
    println!("{}   Ownership Rules:{} ", SUB_HEADER_STYLE, RESET);
    println!(
        "{}    - Each value has a single owner.{}",
        COMMENT_STYLE, RESET
    );
    println!(
        "{}    - When the owner goes out of scope, the value is dropped.{}",
        COMMENT_STYLE, RESET
    );
    println!(
        "{}    - Values can be moved (transferred) to another owner.{}\n",
        COMMENT_STYLE, RESET
    );

    // Borrowing
    println!("{}   Borrowing{} ", SUB_HEADER_STYLE, RESET);
    println!("    fn main() {{");
    println!("        let s1 = String::from(\"hello\");");
    println!(
        "        let len = calculate_length(&s1); {}// borrow s1{}",
        COMMENT_STYLE, RESET
    );
    println!("    }}\n");

    println!("    fn calculate_length(s: &String) -> usize {{");
    println!("        s.len()");
    println!(
        "{}    }} // s goes out of scope, but because it does not{}",
        COMMENT_STYLE, RESET
    );
    println!(
        "{}       // have ownership, nothing happens{}\n",
        COMMENT_STYLE, RESET
    );

    // Mutable References
    println!("{}   Mutable References{} ", SUB_HEADER_STYLE, RESET);
    println!("    fn main() {{");
    println!("        let mut s = String::from(\"hello\");");
    println!(
        "        change(&mut s); {}// mutable borrow{}",
        COMMENT_STYLE, RESET
    );
    println!("    }}\n");

    println!("    fn change(some_string: &mut String) {{");
    println!("        some_string.push_str(\", world\");");
    println!("    }}\n");

    // --- STRUCTS ---
    println!(
        "{}----------------------------------------------------------{}",
        URL_STYLE, RESET
    );
    println!("{}>>> Structs <<<{}\n", HEADER_STYLE, RESET);

    // Defining Structs
    println!("{}   Defining Structs{} ", SUB_HEADER_STYLE, RESET);
    println!("    struct User {{");
    println!("        username: String,");
    println!("        email: String,");
    println!("        sign_in_count: u64,");
    println!("        active: bool,");
    println!("    }}\n");

    // Creating an Instance
    println!("{}   Creating an Instance{} ", SUB_HEADER_STYLE, RESET);
    println!("    let user1 = User {{");
    println!("        username: String::from(\"someone@example.com\"),");
    println!("        email: String::from(\"someone@example.com\"),");
    println!("        sign_in_count: 1,");
    println!("        active: true,");
    println!("    }};\n");

    // Tuple Structs
    println!("{}   Tuple Structs{} ", SUB_HEADER_STYLE, RESET);
    println!("    struct Color(i32, i32, i32);");
    println!("    let black = Color(0, 0, 0);\n");

    // --- ENUMS ---
    println!(
        "{}----------------------------------------------------------{}",
        URL_STYLE, RESET
    );
    println!("{}>>> Enums <<<{}\n", HEADER_STYLE, RESET);

    // Defining and using Enums
    println!("{}   Defining and using Enums{} ", SUB_HEADER_STYLE, RESET);
    println!("    enum IpAddrKind {{");
    println!("        V4,");
    println!("        V6,");
    println!("    }}\n");

    println!("    let four = IpAddrKind::V4;");
    println!("    let six = IpAddrKind::V6;\n");

    // Enums with Data
    println!("{}   Enums with Data{} ", SUB_HEADER_STYLE, RESET);
    println!("    enum Message {{");
    println!("        Quit,");
    println!("        Move {{ x: i32, y: i32 }},");
    println!("        Write(String),");
    println!("        ChangeColor(i32, i32, i32),");
    println!("    }}\n");

    println!("    let m = ");
    println!("    Message::Write(String::from(\"hello\"));\n");

    // --- PATTERN MATCHING ---
    println!(
        "{}----------------------------------------------------------{}",
        URL_STYLE, RESET
    );
    println!("{}>>> Pattern Matching <<<{}\n", HEADER_STYLE, RESET);
    println!("    {}francescociulla.com{}\n", URL_STYLE, RESET);

    // Match Statement
    println!("{}   Match Statement{} ", SUB_HEADER_STYLE, RESET);
    println!("    match some_value {{");
    println!("        Some(x) => println!(\"The value is: {{}}\", x),");
    println!("        None => println!(\"No value\"),");
    println!("    }}\n");

    // If Let
    println!("{}   If Let{} ", SUB_HEADER_STYLE, RESET);
    println!("    if let Some(x) = some_value {{");
    println!("        println!(\"The value is: {{}}\", x);");
    println!("    }}\n");

    // --- ERROR HANDLING ---
    println!(
        "{}----------------------------------------------------------{}",
        URL_STYLE, RESET
    );
    println!("{}>>> Error Handling <<<{}\n", HEADER_STYLE, RESET);

    // Result and Option
    println!("{}   Result and Option{} ", SUB_HEADER_STYLE, RESET);
    println!("    let result: Result<i32, &str> = Ok(200);");
    println!("    let option: Option<i32> = Some(10);\n");

    // Unwrapping
    println!("{}   Unwrapping{} ", SUB_HEADER_STYLE, RESET);
    println!("{}    // panics if result is Err{}", COMMENT_STYLE, RESET);
    println!("    let x = result.unwrap();\n");

    println!(
        "{}    // defaults to 0 if option is None{}",
        COMMENT_STYLE, RESET
    );
    println!("    let y = option.unwrap_or(0);\n");

    // Error Propagation
    println!("{}   Error Propagation{} ", SUB_HEADER_STYLE, RESET);
    println!("    fn read_file() -> Result<String, io::Error> {{");
    println!("        let mut f = File::open(\"hello.txt\")?;");
    println!("        let mut s = String::new();");
    println!("        f.read_to_string(&mut s)?;");
    println!("        Ok(s)");
    println!("    }}\n");

    // --- TRAITS ---
    println!(
        "{}----------------------------------------------------------{}",
        URL_STYLE, RESET
    );
    println!("{}>>> Traits <<<{}\n", HEADER_STYLE, RESET);

    // Defining a Trait
    println!("{}   Defining a Trait{} ", SUB_HEADER_STYLE, RESET);
    println!("    trait Summary {{");
    println!("        fn summarize(&self) -> String;");
    println!("    }}\n");

    // Implementing a Trait
    println!("{}   Implementing a Trait{} ", SUB_HEADER_STYLE, RESET);
    println!("    struct Article {{");
    println!("        headline: String,");
    println!("        content: String,");
    println!("    }}\n");

    println!("    impl Summary for Article {{");
    println!("        fn summarize(&self) -> String {{");
    println!("            format!(\"{{}}, {{}}\", self.headline, self.content)");
    println!("        }}");
    println!("    }}\n");

    // Default Implementation
    println!("{}   Default Implementation{} ", SUB_HEADER_STYLE, RESET);
    println!("    trait Summary {{");
    println!("        fn summarize(&self) -> String {{");
    println!("            String::from(\"(Read more ...)\")");
    println!("        }}");
    println!("    }}\n");

    // --- GENERICS ---
    println!(
        "{}----------------------------------------------------------{}",
        URL_STYLE, RESET
    );
    println!("{}>>> Generics <<<{}\n", HEADER_STYLE, RESET);

    // Generic Function
    println!("{}   Generic Function{} ", SUB_HEADER_STYLE, RESET);
    println!("    fn largest<T: PartialOrd>(list: &[T]) -> &T {{");
    println!("        let mut largest = &list[0];");
    println!("        for item in list {{");
    println!("            if item > largest {{");
    println!("                largest = item;");
    println!("            }}");
    println!("        }}");
    println!("        largest");
    println!("    }}\n");

    // Generic Structs
    println!("{}   Generic Structs{} ", SUB_HEADER_STYLE, RESET);
    println!("    struct Point<T> {{");
    println!("        x: T,");
    println!("        y: T,");
    println!("    }}\n");

    // --- CONCURRENCY ---
    println!(
        "{}----------------------------------------------------------{}",
        URL_STYLE, RESET
    );
    println!("{}>>> Concurrency <<<{}\n", HEADER_STYLE, RESET);

    // Spawning Threads
    println!("{}   Spawning Threads{} ", SUB_HEADER_STYLE, RESET);
    println!("    use std::thread;");
    println!("    let handle = thread::spawn(|| {{");
    println!("        for i in 1..10 {{");
    println!("            println!(\"hi number {{}} from the spawned thread!\", i);");
    println!("        }}");
    println!("    }});");
    println!("    handle.join().unwrap();\n");

    // Channels
    println!("{}   Channels{} ", SUB_HEADER_STYLE, RESET);
    println!("    use std::sync::mpsc;");
    println!("    let (tx, rx) = mpsc::channel();\n");

    println!("    thread::spawn(move || {{");
    println!("        tx.send(String::from(\"hello\")).unwrap();");
    println!("    }});\n");

    println!("    let received = rx.recv().unwrap();");
    println!("    println!(\"Got: {{}}\", received);\n");

    // --- MEMORY MANAGEMENT ---
    println!(
        "{}----------------------------------------------------------{}",
        URL_STYLE, RESET
    );
    println!("{}>>> Memory Management <<<{}\n", HEADER_STYLE, RESET);

    // Smart Pointers
    println!(
        "{}   Smart Pointers (Box, Rc, RefCell){} ",
        SUB_HEADER_STYLE, RESET
    );
    println!("    let b = Box::new(5);");
    println!("    let rc = Rc::new(5);");
    println!("    let refcell = RefCell::new(5);\n");

    // Dereferencing
    println!("{}   Dereferencing{} ", SUB_HEADER_STYLE, RESET);
    println!("    let x = 5;");
    println!("    let y = &x;");
    println!(
        "    assert_eq!(5, *y); {}// dereferencing\n{}",
        COMMENT_STYLE, RESET
    );

    // --- MACROS ---
    println!(
        "{}----------------------------------------------------------{}",
        URL_STYLE, RESET
    );
    println!("{}>>> Macros <<<{}\n", HEADER_STYLE, RESET);

    // Basic Macro Definition
    println!("{}   Basic Macro Definition{} ", SUB_HEADER_STYLE, RESET);
    println!("    macro_rules! say_hello {{");
    println!("        () => {{");
    println!("            println!(\"Hello!\");");
    println!("        }};");
    println!("    }}");
    println!("");
    println!("    say_hello!();\n");

    // --- LIFETIMES ---
    println!(
        "{}----------------------------------------------------------{}",
        URL_STYLE, RESET
    );
    println!("{}>>> Lifetimes <<<{}\n", HEADER_STYLE, RESET);

    // Basic Lifetime Annotation
    println!("{}   Basic Lifetime Annotation{} ", SUB_HEADER_STYLE, RESET);
    println!("    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {{");
    println!("        if x.len() > y.len() {{");
    println!("            x");
    println!("        }} else {{");
    println!("            y");
    println!("        }}");
    println!("    }}\n");

    // --- CARGO AND CRATES ---
    println!(
        "{}----------------------------------------------------------{}",
        URL_STYLE, RESET
    );
    println!("{}>>> Cargo and Crates <<<{}\n", HEADER_STYLE, RESET);

    // Basic Commands
    println!("{}   Basic Commands{} ", SUB_HEADER_STYLE, RESET);
    println!(
        "{}    # Create a new Rust project in a dir 'project_name'{}",
        COMMENT_STYLE, RESET
    );
    println!("    cargo new project_name");
    println!(
        "{}    # Compile the current project{}",
        COMMENT_STYLE, RESET
    );
    println!("    cargo build");
    println!(
        "{}    # Compile and run the current project{}",
        COMMENT_STYLE, RESET
    );
    println!("    cargo run");
    println!(
        "{}    # Run the tests defined in the project{}",
        COMMENT_STYLE, RESET
    );
    println!("    cargo test");
    println!(
        "{}    # Generate doc for the project, open it in a browser{}",
        COMMENT_STYLE, RESET
    );
    println!("    cargo doc --open\n");

    // Adding Dependencies
    println!("{}   Adding Dependencies{} ", SUB_HEADER_STYLE, RESET);
    println!("    {} [in Cargo.toml]{}", URL_STYLE, RESET);
    println!("    [dependencies]");
    println!(
        "{}    # Specify the name of the crate & the version you want to use{}",
        COMMENT_STYLE, RESET
    );
    println!("    rand = \"0.8\"");
    println!(
        "{}    # To work with random numbers{}",
        COMMENT_STYLE, RESET
    );
    println!("    serde = \"1.0\"");
    println!(
        "{}    # A serialization/deserialization crate{}",
        COMMENT_STYLE, RESET
    );
    println!("    regex = \"1.5\"");
    println!("{}    # For working with regex{}\n", COMMENT_STYLE, RESET);

    // Final Footer Separator
    println!(
        "{}=========================================================={}",
        HEADER_STYLE, RESET
    );
}
