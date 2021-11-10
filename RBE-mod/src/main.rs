use deeply::nested::function as other_function;

fn main() {
    other_function();
    println!("Entering block");
    {
        use deeply::nested::function;
        function();
    }
    function();
}

fn function() {
    println!("called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`")
        }
    }
}

