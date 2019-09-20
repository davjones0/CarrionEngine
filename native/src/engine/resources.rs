pub struct Map {
    pub placement: Vec<MapTile>
}

impl Map {
    pub fn new() -> Map {
        Map {
            placement: Vec::new()
        }
    }

    pub fn generate(&mut self) {
        for x in 0..100 {
            for y in 0..100 {
                self.placement.push(MapTile::new());
            }
        }
    }
}

#[derive(Default)]
pub struct MapTile {
    pub layers: Vec<String>,
    pub name: String,
    pub description: String,
}

impl MapTile {
    pub fn new() -> MapTile {
        MapTile { 
            layers: Vec::new(),
            name: String::from("PlaceHolder"),
            description: String::from("Place"),
        }
    }
}

