#[derive(Debug, Clone, Default)]
pub struct Model {
    pub introductie: String,
    pub vraag: String,
    pub antwoord: Vec<String>,
    pub verklaring: String,
    pub materiaal: Vec<String>,
    pub voorbereiding: String,
}
