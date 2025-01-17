use std::env;
use std::fs;

use ansi_term::Style;
use draw::render;
use draw::SvgRenderer;
use gsolve::draw::draw;
use gsolve::order::bfs_order;
use gsolve::parse::parse_document;
use gsolve::solve::brute_solve;
// use gsolve::solve::dist_solve;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    let file_path = args.get(1)
    .ok_or("no file path specified")?;

    let contents = fs::read_to_string(file_path)
    .map_err(|e| {format!("{e}")})?;

    let mut index = parse_document(contents)?;

    println!("\n{}", Style::new().underline().paint("Constraints"));
    for c in index.constraints() {
        println!("{}", c);
    }

    let order = bfs_order(&mut index);

    println!("\n{}", Style::new().underline().paint("Constraints by Point"));
    for (id, cs) in order.iter().enumerate() {
        println!("{}:", index.get_point(&id));
        for &c in cs {
            println!(" {}", index.get_constraint(c));
        }
    }
    
    let positions = brute_solve(&mut index, order)?;

    println!("\n{}", Style::new().underline().paint("Positions"));
    for (point, pos) in positions.iter() {
        println!("{}: {}", point, pos);
    }

    let canvas = draw(positions, 96.0);

    render::save(
        &canvas,
        "figure.svg",
        SvgRenderer::new()
    ).or(Err("render failed".to_string()))?;

    Ok(())
}