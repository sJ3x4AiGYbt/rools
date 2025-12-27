pub fn run() {
    println!("Hello Windows! 🪟");
    
    // Informations spécifiques à Windows
    println!("\n--- Informations spécifiques Windows ---");
    
    if let Ok(username) = std::env::var("USERNAME") {
        println!("Utilisateur : {}", username);
    }
    
    if let Ok(computername) = std::env::var("COMPUTERNAME") {
        println!("Nom de l'ordinateur : {}", computername);
    }
    
    if let Ok(userprofile) = std::env::var("USERPROFILE") {
        println!("Profil utilisateur : {}", userprofile);
    }
    
    // Architecture
    let arch = std::env::consts::ARCH;
    println!("Architecture : {}", arch);
}