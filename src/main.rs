use bevy::prelude::*;
use rand::Rng;

fn main() {
    App::new()
        .add_systems(Startup, spawn_palyers)
        .add_systems(Update, (name_errosion, name_printing).chain())
        .run();
}


#[derive(Component)]
struct Player{ 
    id:i32
}
impl Player {
    fn new(id: i32) -> Self {
        Self { id }
    }
}


#[derive(Component)]
struct Name{
    pub name: String
}
impl Name {
    fn new(name: String) -> Self {
        Self { name }
    }
}








fn spawn_palyers(mut commands: Commands) {
    for i in 0..10 {
        commands.spawn((
            Player::new(i),
            Name::new("We can then add people to our World using a \"startup system\". Startup systems are just like normal systems".to_string())),
        );
    }
}



fn name_errosion(mut query: Query<&mut Name>) {
    for mut name in &mut query {
        let index = rand::thread_rng().gen_range(0..name.name.len());
        let replacement_char = rand::thread_rng().gen_range('a'..'z');
        let replacement = replacement_char.to_string();

        name.name.replace_range(index..index + 1, replacement.as_str());
    }
}
fn name_printing(query: Query<(&Name, &Player)>) {
    for (name, player) in query.iter() {
        println!("Player: {} name: {}", player.id, name.name);
    }
}
