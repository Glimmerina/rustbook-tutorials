#[derive(Debug, PartialEq, Copy, Clone)]
// Enum representing shirt colours.
// I also changed the spelling to British English. God save the King.
enum ShirtColour {
    Red,
    Blue,
}

// Struct representing an inventory of shirts.
struct Inventory {
    shirts: Vec<ShirtColour>,
}
// Implements mentods for the inventory.
// The functions manage the giveaway based on preference or availability.
impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColour>) -> ShirtColour {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColour {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColour::Red => num_red += 1,
                ShirtColour::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColour::Red
        } else {
            ShirtColour::Blue
        }
    }
}
// Main function to demonstrate the giveaway functionality.
// It creates an inventory and simulates user preferences.
fn main() {
    let store = Inventory {
        shirts: vec![ShirtColour::Blue, ShirtColour::Red, ShirtColour::Blue],
    };

    let user_pref1 = Some(ShirtColour::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}

