// This file is to test the file.exec command
#[derive(Debug)]
enum Location {
    Heap,
    Stack,
}

#[derive(Debug)]
struct Variable {
    location: Location,
    pointer: bool,
    value: &'static str,
}

impl Variable {
    fn is_pointer(&self) -> bool {
        self.pointer
    }
    fn what_value(&self) -> &str {
        self.value
    }
    fn where_at(&self) -> &str {
        match self.location {
            self::Location::Heap => "Heap",
            self::Location::Stack => "Stack",
        }
    }
}

fn main() {
    // Variable Definitions
    let var1 = Variable {
        location: Location::Heap,
        pointer: false,
        value: "Hello World!",
    };
    let var2 = Variable {
        location: Location::Stack,
        pointer: true,
        value: "0x7ffd976f1e48",
    };

    println!();
    
    // Variable 1
    println!("var1 located on the {}", var1.where_at());
    
    if var1.is_pointer() == true {
        println!("var1 is a pointer");
    } else {
        println!("var1 is not a pointer");
    };

    println!("var1 value is: {}", var1.what_value());

    println!();

    // Variable 2
    println!("var2 located on the {}", var2.where_at());
    
    if var2.is_pointer() == true {
        println!("var2 is a pointer");
    } else {
        println!("var2 is not a pointer");
    };

    println!("var2 value is: {}", var2.what_value());
}