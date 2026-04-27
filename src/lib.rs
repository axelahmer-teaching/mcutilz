use std::io::{self, BufRead, Read, Write};

use anyhow::{Result, bail};
use mcrs::chunk::ChunkStream;
use mcrs::{Block, Coordinate};

pub fn write_data(
    file: &mut impl Write,
    chunk: &mut ChunkStream<'_>,
    player_pos: Coordinate,
) -> Result<()> {
    let mut index = 0;
    let origin = chunk.origin();
    let size = chunk.size();
    while let Some(item) = chunk.next()? {
        let coord = origin + size.index_to_offset(index);
        let block = item.block();
        let rel_x = coord.x - player_pos.x;
        let rel_y = coord.y - player_pos.y;
        let rel_z = coord.z - player_pos.z;
        writeln!(
            file,
            "{},{},{}: {}:{}",
            rel_x, rel_y, rel_z, block.id, block.modifier
        )?;
        index += 1;
    }

    Ok(())
}

pub fn read_data<R: Read>(file: &mut R) -> Result<Vec<(Coordinate, Block)>> {
    let reader = io::BufReader::new(file);
    let mut blocks = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() != 2 {
            bail!("Invalid file format: {}", line);
        }

        let coord_parts: Vec<&str> = parts[0].split(',').collect();
        if coord_parts.len() != 3 {
            bail!("Invalid coordinates: {}", parts[0]);
        }

        let x: i32 = coord_parts[0].parse()?;
        let y: i32 = coord_parts[1].parse()?;
        let z: i32 = coord_parts[2].parse()?;

        let block_parts: Vec<&str> = parts[1].split(':').collect();
        let (id, modifier) = if block_parts.len() == 2 {
            (block_parts[0].parse()?, block_parts[1].parse()?)
        } else if block_parts.len() == 1 {
            (block_parts[0].parse()?, 0)
        } else {
            bail!("Invalid block: {}", parts[1]);
        };

        blocks.push((Coordinate::new(x, y, z), Block::new(id, modifier)));
    }

    Ok(blocks)
}
