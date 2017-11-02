use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize)]
pub(crate) struct GameConfig {
    pub(crate) n_bullets: i32,
}

impl GameConfig {
    pub(crate) fn parse(filename: String) -> Self {
        let mut f = File::open(filename).expect("cannot open config file");
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).expect(
            "cannot read from config file",
        );

        ::toml::from_str(&buffer).expect("cannot parse config")
    }
}
