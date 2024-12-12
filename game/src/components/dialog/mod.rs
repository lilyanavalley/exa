

// #[test]
// pub fn test_pull_ftl_file() -> std::io::Result<()> {

//     let mut game = Game::default();
//     game.with_app_data_dir(PathBuf::from("../data"));
//     let ftl_path = game.app_data_dir().join("languages/en-us.ftl");

//     let mut buffer = String::new();
//     let mut file = File::open(ftl_path)?;
//     let _ = file.read_to_string(&mut buffer);

//     let fluent_resource = FluentResource::try_new(buffer)
//         .expect("FTL parsing failed");

//     let mut fluent_bundle = FluentBundle::new(vec![langid!("en-us")]);
//     fluent_bundle.add_resource(fluent_resource)
//         .expect("Add FTL resource failed");

//     let message = fluent_bundle.get_message("coremenu")
//         .expect("Failed to get message by its ID");

//     let test_errors: Vec<FluentError> = Vec::new();
    
//     println!("{:?}", message.value().unwrap());

//     Ok(())

// }

#[derive(Debug, Default)]
pub struct Dialog {

}
