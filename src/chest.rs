pub struct Chest {
    pub x: i32,
    pub y: i32,
    pub items: Vec<Item>,
    pub opened: bool,
}

Pub enum Item {
    Wood,
    Stone,
    Gold,
}

pub fn get_chest() {
    let chest = Chest {
        x: 10,
        y: 5,
        items: vec![
            Items::Wood,
            Items::Gold,
        ],
        opened: false,
    };

}