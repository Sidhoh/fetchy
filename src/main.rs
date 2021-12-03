use nixinfo;
use ansi_term::Colour::Purple;
use ansi_term::Colour::Blue;



fn main() {
    let distro = nixinfo::distro();
    let shell = nixinfo::env("SHELL");
    let uptime = nixinfo::uptime();
    let kernel = nixinfo::kernel();
    let wm = nixinfo::environment();

    println!("\n  {}  {}  {}", Blue.paint("os"), Purple.paint("~"), &distro.unwrap().replace("\"", ""));
    println!("  {}  {}  {}", Blue.paint("sh"), Purple.paint("~"), &shell.unwrap().replace("/usr/bin/", ""));
    println!("  {}  {}  {}", Blue.paint("up"), Purple.paint("~"), &uptime.unwrap());
    println!("  {} {}  {}", Blue.paint("ker"), Purple.paint("~"), &kernel.unwrap());
    println!("  {}  {}  {}", Blue.paint("wm"), Purple.paint("~"), &wm.unwrap());

}
