use toml::Table;
use toml::Value;
use serde::Deserialize;

const raw_toml: &str = r#"
[[subclasses]]
class = "Artificier"
subclasses = ["Alchemist", "Armorer", "Artillerist", "Battle Smith"]

[[subclasses]]
class = "Barbarian"
subclasses = ["Path of the Ancestral Guardian", "Path of the Battlerager",
            "Path of the Beast", "Path of the Berserker", "Path of the Giant",
            "Path of the Storm Herald", "Path of the Wild Heart", "Path of Wild Magic",
            "Path of the Zealot", "Path of the World Tree"]

[[subclasses]]
class = "Bard"
subclasses = ["College of Creation", "College of Eloquence", "College of Lore",
            "College of Spirits", "College of Swords", "College of Valor", "College of Whispers",
            "College of Dance"]

[[subclasses]]
class = "Cleric"
subclasses = ["Arcana Domain", "Death Domain", "Forge Domain", "Grave Domain", "Life Domain",
            "Knowledge Domain", "Light Domain", "Nature Domain", "Order Domain", "Peace Domain",
            "Tempest Domain", "Trickery Domain", "Twilight Domain", "War Domain"]

[[subclasses]]
class = "Druid"
subclasses = ["Circle of Dreams", "Circle of the Land", "Circle of the Moon", "Circle of the Shepherd",
            "Circle of the Spores", "Circle of the Stars", "Circle of the Wildfire", "Circle of the Sea"]

[[subclasses]]
class = "Fighter"
subclasses = ["Arcane Archer", "Battle Master", "Cavalier", "Champion", "Echo Knight", "Eldritch Knight",
            "Psi Warrior", "Purple Dragon Knight", "Rune Knight", "Samurai"]

[[subclasses]]
class = "Monk"
subclasses = ["Way of the Ascendant Dragon", "Way of the Astral Self", "Way of the Drunken Master",
            "Way of the Four Elements", "Way of the Kensei", "Way of the Long Death", "Way of Mercy",
            "Way of the Open Hand", "Way of the Shadow", "Way of the Sun Soul"]

[[subclasses]]
class = "Paladin"
subclasses = ["Oath of Conquest", "Oath of Devotion", "Oath of Glory", "Oath of Redemption",
            "Oath of the Ancients", "Oath of the Crown", "Oath of the Watchers", "Oath of Vengeance",
            "Oathbreaker"]

[[subclasses]]
class = "Ranger"
subclasses = ["Beast Master", "Drake Warden", "Fey Wanderer", "Gloom Stalker", "Horizon Walker",
            "Hunter", "Monster Slayer", "Swarmkeeper"]

[[subclasses]]
class = "Rogue"
subclasses = ["Arcane Trickster", "Assassin", "Inquisitive", "Mastermind", "Phantom",
            "Scout", "Soulknife", "Swashbuckler", "Thief"]

[[subclasses]]
class = "Sorcerer"
subclasses = ["Abberant Mind", "Clockwork Soul", "Divine Soul", "Draconic Bloodline",
            "Lunar Sorcery", "Shadow Magic", "Storm Sorcery", "Wild Magic"]

[[subclasses]]
class = "Warlock"
subclasses = ["Archfey Patron", "Celestial", "Fathomless", "Fiend", "Genie", "Great Old One",
            "Hexblade", "Undead", "Undying"]

[[subclasses]]
class = "Wizard"
subclasses = ["Bladesinger", "Chronurgy", "Graviturgy", "Order of Scribes", "School of Abjuration",
            "School of Conjuration", "School of Divination", "School of Enchantment", "School of Evocation",
            "School of Illusion", "School of Necromancy", "School of Transmutation", "War Magic"]

"#;

#[derive(Debug, Deserialize)]
pub struct CharacterTOML {
    subclasses: Vec<Subclasses>,
}

#[derive(Debug, Deserialize)]
pub struct Subclasses {
    class: String,
    subclasses: Vec<String>
}

fn main() {
    let value = raw_toml.parse::<Table>().unwrap();
    println!("{:?}", &value);
    println!();

    for entry in value {
        match entry {
            (_, Value::Array(val)) => println!("{:?}", &val),
            _ => {},
        }
    }
    println!();

    let parsed_char_toml: CharacterTOML = toml::from_str(raw_toml).unwrap();

    println!("{:?}", parsed_char_toml);
}
