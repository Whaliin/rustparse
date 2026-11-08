mod solver;
mod tokenizer;
mod types;

use std::io::{self, Write};
use solver::Solver;

fn main() {
    let mut buffer = String::new();
	let mut solver = Solver::default();

    loop {
        print!("> ");
        io::stdout().flush().unwrap(); // Ensures the "> " prompt prints immediately

        buffer.clear();
        // Reads from console until user presses Enter:
        if io::stdin().read_line(&mut buffer).is_err() {
            break;
        }

        let input = buffer.trim(); // Strips \r\n

		// Parse commands
		if input.starts_with(":") {
			match input {
				":deg" => solver.options.trig_deg = true,
				":rad" => solver.options.trig_deg = false,
				":vars" => {
					let vars = solver.variables();

					if vars.len() > 0 {
						for (key, value) in vars {
							println!(" {} = {}", key, value);
						}
					} else {
						println!("-")
					}
				}
				":clear" => solver.clear_variables(),
				":help" => {
					println!("Available commands: ");
					println!(" :deg - Use degrees");
					println!(" :rad - Use radians");
					println!(" :vars - View variables");
					println!(" :clear - Clear variables");
					println!(" :exit/:quit - Exit program");
				}
				":exit" | ":quit" => break,
				_ => println!("Unknown command.")
			}
			continue;
		}

        match solver.solve(input) {
            Ok(res) => {
				println!("= {res}");
				// save the answer as "ans"
				solver.set_variable("ans", res);
			},
            Err(e) => eprintln!("Solver error; {e}"),
        }
    }
}