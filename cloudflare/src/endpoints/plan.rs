#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
/// Free plans won't have a Frequency, so most responses should accept Option instead.
pub enum Frequency {
    #[serde(rename = "")]
    Nil,
    Weekly,
    Monthly,
    Quarterly,
    Yearly,
}

#[derive(Deserialize, Debug)]
pub struct Plan {
    /// Plan identifier tag
    pub id: String,
    /// The plan name
    pub name: String,
    /// The price of the subscription that will be billed, in US dollars
    pub price: f64,
    /// The monetary unit in which pricing information is displayed
    pub currency: String,
    /// The frequency at which you will be billed for this plan
    pub frequency: Option<Frequency>,
    /// A 'friendly' identifier to indicate to the UI what plan the object is
    pub legacy_id: String,
    /// If the zone is subscribed to this plan
    pub is_subscribed: bool,
    /// If the zone is allowed to subscribe to this plan
    pub can_subscribe: bool,
}
