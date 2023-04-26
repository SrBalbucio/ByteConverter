use std::io;

use bytes::Bytes;

fn main() {
    println!(
        "Use 'write' para transformar Texto em Bytes"
    );
    let mut command = String::new();
    io::stdin().read_line(&mut command);
    if (command.contains("write")) {
        println!("Digite o texto aqui:");
        command.clear();
        io::stdin().read_line(&mut command);
        let mut byte = Bytes::from(command).to_ascii_lowercase();
        println!("Resultado: ");
        for i in byte{
            print!(" {}", i);
        }
        print!("\n");
    }
}
