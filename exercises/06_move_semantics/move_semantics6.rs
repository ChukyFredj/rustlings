// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

/// Ici je ne pouvour pas changer le code, je dois juste ajouter ou supprimer des références. Pour que lee codee marchee
/// je dois ajouter des références aux fonctions get_char et string_uppercase.
/// j'ai ajouté & à data dans la fonction get_char et ajouté mut à data dans la fonction string_uppercase.pour stocker les données upercase dans data elle meme.

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();
    println!("{}", data);
}
