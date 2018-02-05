fn main() {
    println!("{} days", 31);
    println!("{0} is her. {1} is him", "Alice", "Bob");
    println!("{subject} {verb} {object}", subject="Bob", 
                                         verb="Hosts", object="Game Shows");
    println!("{:b} of {:b} people know binary, the other half doesn't", 1, 2);
    println!("{number:>width$}", number=1, width=6);
    println!("{number:>0width$}", number=1, width=6);
    println!("My name is {0}, {1} {0}", "Bond", "Jjaime");
    let pi_estimate = 3.1233456;
    println!("{:.3}", pi_estimate )

}
