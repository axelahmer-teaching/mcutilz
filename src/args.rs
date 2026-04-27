use clap::{Parser, Subcommand};

pub const DEFAULT_DX: u32 = 20;
pub const DEFAULT_DY: u32 = 10;
pub const DEFAULT_DZ: u32 = 20;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Clear a 3D block region (set every block to air)
    ///
    /// The cleared region is centered on the player's current position.
    Clear {
        /// Set a uniform radius across all dimensions (overrides defaults)
        #[arg(short, long)]
        radius: Option<u32>,
        /// Radius in the X dimension (east/west) [default: 20]
        #[arg(long)]
        dx: Option<u32>,
        /// Radius in the Y dimension (up/down) [default: 10]
        #[arg(long)]
        dy: Option<u32>,
        /// Radius in the Z dimension (north/south) [default: 20]
        #[arg(long)]
        dz: Option<u32>,
    },

    /// Store a 3D block region to a text file centered around the player
    ///
    /// File will include relative coordinates and block data in a plain text format `x,y,z: id:modifier`,
    /// making it easy to generate regions for autograders.
    Save {
        /// Name of text file to save to
        filename: String,
        /// Set a uniform radius across all dimensions (overrides defaults)
        #[arg(short, long)]
        radius: Option<u32>,
        /// Radius in the X dimension (east/west) [default: 20]
        #[arg(long)]
        dx: Option<u32>,
        /// Radius in the Y dimension (up/down) [default: 10]
        #[arg(long)]
        dy: Option<u32>,
        /// Radius in the Z dimension (north/south) [default: 20]
        #[arg(long)]
        dz: Option<u32>,
    },

    /// Load a 3D block region from a text file
    ///
    /// The saved region is loaded relative to the player's current position within the world.
    Load {
        /// Name of text file to load from
        filename: String,
    },
}
