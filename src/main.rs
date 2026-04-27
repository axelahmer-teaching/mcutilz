mod args;

use std::fs;
use std::io;

use anyhow::Result;
use clap::Parser;
use mcrs::{Block, Connection, Coordinate};

use crate::args::Command;
use mcutilz::{read_data, write_data};

fn main() -> Result<()> {
    let args = args::Args::parse();

    let mut mc = Connection::new().expect("Failed to connect to Minecraft server");

    match args.command {
        Command::Clear { radius, dx, dy, dz } => {
            let player_pos = mc.get_player_position()?;
            
            let rx = dx.or(radius).unwrap_or(args::DEFAULT_DX) as i32;
            let ry = dy.or(radius).unwrap_or(args::DEFAULT_DY) as i32;
            let rz = dz.or(radius).unwrap_or(args::DEFAULT_DZ) as i32;

            let origin = Coordinate::new(
                player_pos.x - rx,
                player_pos.y - ry,
                player_pos.z - rz,
            );
            let bound = Coordinate::new(
                player_pos.x + rx,
                player_pos.y + ry,
                player_pos.z + rz,
            );

            let (origin, bound) = sort_corners(origin, bound);

            let chunk = mc.get_blocks(origin, bound)?;
            let size = origin.size_between(bound);

            for i in 0..size.volume() {
                let coord = origin + size.index_to_offset(i);
                let block = Block::AIR;
                let current_block = chunk
                    .get_worldspace(coord)
                    .expect("Chunk should contain coordinate");
                if block != current_block {
                    mc.set_block(coord, block)?;
                }
            }

            println!("Successfully cleared {:?} chunk at {}.", size, origin);
        }

        Command::Save { filename, radius, dx, dy, dz } => {
            let player_pos = mc.get_player_position()?;
            
            let rx = dx.or(radius).unwrap_or(args::DEFAULT_DX) as i32;
            let ry = dy.or(radius).unwrap_or(args::DEFAULT_DY) as i32;
            let rz = dz.or(radius).unwrap_or(args::DEFAULT_DZ) as i32;

            let origin = Coordinate::new(
                player_pos.x - rx,
                player_pos.y - ry,
                player_pos.z - rz,
            );
            let bound = Coordinate::new(
                player_pos.x + rx,
                player_pos.y + ry,
                player_pos.z + rz,
            );

            let (origin, bound) = sort_corners(origin, bound);

            let file = fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(filename)?;
            let mut writer = io::BufWriter::new(file);

            let mut chunk = mc.get_blocks_stream(origin, bound)?;
            write_data(&mut writer, &mut chunk, player_pos)?;

            println!(
                "Successfully saved {:?} chunk at {}.",
                chunk.size(),
                player_pos,
            );
        }

        Command::Load { filename } => {
            let file = fs::OpenOptions::new().read(true).open(filename)?;
            let mut reader = io::BufReader::new(file);

            let blocks = read_data(&mut reader)?;

            if blocks.is_empty() {
                println!("Loaded empty chunk.");
                return Ok(());
            }

            let new_origin = mc.get_player_position()?;

            // Compute size and bound
            let mut min_coord = blocks[0].0;
            let mut max_coord = blocks[0].0;
            for (coord, _) in &blocks {
                min_coord.x = std::cmp::min(min_coord.x, coord.x);
                min_coord.y = std::cmp::min(min_coord.y, coord.y);
                min_coord.z = std::cmp::min(min_coord.z, coord.z);
                max_coord.x = std::cmp::max(max_coord.x, coord.x);
                max_coord.y = std::cmp::max(max_coord.y, coord.y);
                max_coord.z = std::cmp::max(max_coord.z, coord.z);
            }

            let shifted_origin = Coordinate::new(
                min_coord.x + new_origin.x,
                min_coord.y + new_origin.y,
                min_coord.z + new_origin.z,
            );
            let shifted_bound = Coordinate::new(
                max_coord.x + new_origin.x,
                max_coord.y + new_origin.y,
                max_coord.z + new_origin.z,
            );

            let chunk = mc.get_blocks(shifted_origin, shifted_bound)?;

            for (coord, block) in blocks {
                let shifted_coord = Coordinate::new(
                    coord.x + new_origin.x,
                    coord.y + new_origin.y,
                    coord.z + new_origin.z,
                );
                let current_block = chunk
                    .get_worldspace(shifted_coord)
                    .unwrap_or(mcrs::Block::AIR);
                if block != current_block {
                    mc.set_block(shifted_coord, block)?;
                }
            }

            println!("Successfully loaded chunk offset to {}.", new_origin);
        }
    }

    Ok(())
}

fn sort_corners(a: Coordinate, b: Coordinate) -> (Coordinate, Coordinate) {
    (a.min(b), a.max(b))
}
