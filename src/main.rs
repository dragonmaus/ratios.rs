use std::collections::HashMap;

program::main!("ratios");

fn usage_line(program_name: &str) -> String {
    format!("Usage: {} x y", program_name)
}

fn program(name: &str) -> program::Result {
    let args = program::args().split_off(1);

    if args.len() != 2 {
        eprintln!("{}", usage_line(name));
        return Ok(1);
    }

    let xmax = args[0].trim().parse::<u64>()?;
    let ymax = args[1].trim().parse::<u64>()?;

    let mut best = HashMap::new();

    for x in 2..=xmax {
        for y in 2..=ymax {
            let gcd = gcd(x, y);
            let rx = x / gcd;
            let ry = y / gcd;
            best.insert(vec![rx, ry], vec![x, y]);
        }
    }

    for (k, v) in &best {
        let r = k[0] as f64 / k[1] as f64;
        println!("{} ({}/{}) => {}x{}", r, k[0], k[1], v[0], v[1]);
    }

    Ok(0)
}

fn gcd(x: u64, y: u64) -> u64 {
    let mut gcd = *(vec![x, y].iter().min().unwrap_or(&1));

    while gcd > 1 {
        if x % gcd == 0 && y % gcd == 0 {
            break;
        }
        gcd -= 1;
    }

    gcd
}
