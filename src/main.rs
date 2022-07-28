/*
 * app_cli is a commandline application.
 * Copyright © 2022 Leopold Meinel & contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see https://github.com/LeoMeinel/app_cli/blob/main/LICENSE
 */

use std::{env, process};

use app_cli::{grep, Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        // Print to standard error stream
        eprintln!("ERROR: Problem parsing arguments! - {}", err);
        process::exit(1);
    });
    if let Err(err) = grep::grep(config) {
        eprintln!("ERROR: {}", err);
        process::exit(1);
    }
}
