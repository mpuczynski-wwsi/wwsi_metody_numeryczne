pub fn run() 
{
    parse("(((()(()))))");
}

fn parse(s: &str)
{
    for c in s.chars() { 
        println!("{}", c);
    }
}
