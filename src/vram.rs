#[derive(Default)]
pub struct Ppu {
    pub lcdc: Lcdc,

    pub ly: u8,
    pub lyc: u8,
    pub sc_x: u8,
    pub sc_y: u8,

    pub w_x: u8,
    pub w_y: u8,

    pub bg_palette: [DmgColor; 4],
    pub obj0_palette: [DmgColor; 4],
    pub obj1_palette: [DmgColor; 4],
}


pub enum DmgColor {
    Black,
    DarkGray,
    LightGray,
    White,
}

impl Default for DmgColor {
    fn default() -> Self {
        DmgColor::White
    }
}

/*
 Bit 7 - LCD Display Enable             (0=Off, 1=On)
 Bit 6 - Window Tile Map Display Select (0=9800-9BFF, 1=9C00-9FFF)
 Bit 5 - Window Display Enable          (0=Off, 1=On)
 Bit 4 - BG & Window Tile Data Select   (0=8800-97FF, 1=8000-8FFF)
 Bit 3 - BG Tile Map Display Select     (0=9800-9BFF, 1=9C00-9FFF)
 Bit 2 - OBJ (Sprite) Size              (0=8x8, 1=8x16)
 Bit 1 - OBJ (Sprite) Display Enable    (0=Off, 1=On)
 Bit 0 - BG/Window Display/Priority     (0=Off, 1=On)
 */
#[derive(Default)]
pub struct Lcdc {
    pub lcd_display_enable: bool,
    pub window_tilemap_select: bool,
    pub window_enabled: bool,
    pub bg_window_tile_data_select1: bool,
    pub bg_tilemap_select1: bool,
    pub tall_sprites: bool,
    pub sprites_enabled: bool,
    pub bg_window_priority: bool,
}

impl Lcdc {
    pub fn to_byte(&self) -> u8 {
        (
           (self.lcd_display_enable          as u8) << 7 |
           (self.window_tilemap_select       as u8) << 6 |
           (self.window_enabled              as u8) << 5 |
           (self.bg_window_tile_data_select1 as u8) << 4 |
           (self.bg_tilemap_select1          as u8) << 3 |
           (self.tall_sprites                as u8) << 2 |
           (self.sprites_enabled             as u8) << 2 |
           (self.bg_window_priority          as u8) << 0
        )
    }

    pub fn from_byte(b: u8) -> Lcdc {
        Lcdc {
            lcd_display_enable:          (b & (1 << 7) != 0),
            window_tilemap_select:       (b & (1 << 6) != 0),
            window_enabled:              (b & (1 << 5) != 0),
            bg_window_tile_data_select1: (b & (1 << 4) != 0),
            bg_tilemap_select1:          (b & (1 << 3) != 0),
            tall_sprites:                (b & (1 << 2) != 0),
            sprites_enabled:             (b & (1 << 2) != 0),
            bg_window_priority:          (b & (1 << 0) != 0),
        }
    }
}

#[derive(Default)]
pub struct Vram {
    pub tileset: Tileset,
    pub bg_tilemap: Tilemap,
    pub window_tilemap: Tilemap,
}

impl Vram {
    pub fn from_bytes(bytes: &[u8]) -> Vram {
        const VRAM_OFFSET: usize = 0x8000;
        Vram {
            tileset: build_tileset(bytes),
            bg_tilemap: build_tilemap(bytes),
            window_tilemap: build_tilemap(bytes), // TODO BUILD REAL WINDOW
        }
    }
}

fn build_tileset(vram: &[u8]) -> Tileset {
    let mut tiles = [Tile { pixels: [0; 64] }; 384];
    for tidx in 0..(256 + 128) {
        let tile_bytes = &vram[(tidx * TILE_SIZE)..(tidx * TILE_SIZE) + TILE_SIZE];
        let tile = build_tile(tile_bytes);
        tiles[tidx] = tile;
    }
    Tileset { tiles }
}

const TILE_SIZE: usize = 16;

fn build_tile(tile_bytes: &[u8]) -> Tile {
    let mut pixels = [0; 64];
    debug_assert_eq!(tile_bytes.len(), 16);
    for y in 0..8 {
        let lo_byte = tile_bytes[2 * y + 0];
        let hi_byte = tile_bytes[2 * y + 1]; // may be inverted
        for x in 0..8 {
            let lo_bit = (lo_byte >> (7 - x)) & 1;
            let hi_bit = (hi_byte >> (7 - x)) & 1;
            let color_idx = (hi_bit << 1) | lo_bit;
            pixels[y * 8 + x] = color_idx;
        }
    }
    Tile { pixels }
}

fn build_tilemap(vram: &[u8]) -> Tilemap {
    let mut tile_idxs = [0; 32 * 32];
    for i in 0..(32 * 32) {
        tile_idxs[i] = vram[0x1800 + i];
    }
    Tilemap { tile_idxs }
}


#[derive(Copy, Clone)]
pub struct Tile {
    pixels: [u8; 64], // despite u8 type it only contains values 0-3
}

impl Default for Tile {
    fn default() -> Self {
        Tile { pixels: [0; 64] }
    }
}

pub struct Oam {
    sprites: [OamEntry; 40]
}
impl Oam {
    pub fn from_bytes(bytes: &[u8]) -> Oam {
        let mut oam: Oam = Default::default();
        for i in 0..40 {
            oam.sprites[i] = OamEntry::from_bytes(&bytes[i..i+4])
        }
        oam
    }
}

impl Default for Oam {
    fn default() -> Self {
        Oam { sprites: [Default::default(); 40] }
    }
}


#[derive(Default, Clone, Copy)]
pub struct OamEntry {
    pos_x: u8,
    pos_y: u8,
    tile_idx: u8,
    flip_x: bool,
    flip_y: bool,

    // if true, draw only on top of white BG pixels, if false on top of everything
    low_priority: bool,

    // 0 or 1 from existing pallettes
    palette1: bool,

}

impl OamEntry {
    pub fn from_bytes(bytes: &[u8]) -> OamEntry {
        let flags = bytes[3];
        OamEntry {
            pos_y: bytes[0],
            pos_x: bytes[1],
            tile_idx: bytes[2],
            low_priority: (flags & (1 << 7)) != 0,
            flip_y: (flags & (1 << 6)) != 0,
            flip_x: (flags & (1 << 5)) != 0,
            palette1: (flags & (1 << 4)) != 0,
        }
    }
}


impl Tile {
    pub fn get_pixel(&self, x: i32, y: i32) -> u8 {
        self.pixels[(y * 8 + x) as usize]
    }
}

pub struct Tileset {
    pub tiles: [Tile; 256 + 128],
}

impl Default for Tileset {
    fn default() -> Self {
        Tileset { tiles: [Default::default(); 256 + 128] }
    }
}

impl Tileset {
    pub fn get_tile_mode_0(&self, idx: u8) -> &Tile {
        &self.tiles[idx as usize]
    }

    pub fn get_tile_mode_1(&self, idx: i8) -> &Tile {
        &self.tiles[(256 + idx as i32) as usize]
    }
}

pub struct Tilemap {
    tile_idxs: [u8; 32 * 32],
}

impl Default for Tilemap {
    fn default() -> Self {
        Tilemap { tile_idxs: [0; 32 * 32] }
    }
}

impl Tilemap {
    pub fn get<'a>(&self, tileset: &'a Tileset, _mode: u8, x: i32, y: i32) -> &'a Tile {
        let idx = self.tile_idxs[(y * 32 + x) as usize];
        tileset.get_tile_mode_0(idx) // FIXME MODE 0 hardcoded for now
    }
}
