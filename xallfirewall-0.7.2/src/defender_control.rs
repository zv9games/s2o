use crossbeam_channel::Sender;

pub fn load_module(tx: Sender<String>) -> Result<(), Box<dyn std::fmt::Debug>> {
    tx.send("defender_control module loaded.".to_string()).map_err(|e| Box::new(e) as Box<dyn std::fmt::Debug>)?;
    println!("Sent: defender_control module loaded.");
    Ok(())
}