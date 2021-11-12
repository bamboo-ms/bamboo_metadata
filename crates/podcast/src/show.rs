use super::Rating;
use crate::episode::Episode;
use bamboo_metadata_util::person::Person;
use isocountry::CountryCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Category {
    // TODO: Replace with enum of supported categories
    pub name: String,
    pub subcategories: Vec<Category>,
}

// "Indicates a show i.e. the podcast itself. It contains the metadata for the show and
// encapsulates all the show episodes as items. Defining multiple podcasts in the same RSS is not
// supported." - 5.2
#[derive(Serialize, Deserialize)]
pub struct Show {
    // "The full name of the show main originator, group or person." - 5.7
    pub author: Option<Person>,
    // TODO: consider reworking the "Rating" type to better accommodate the
    // "Indicates if the podcast contains explicit material in any of its episodes... Clean
    // indicates the podcast in its entirety is suitable to minors with or without edited
    // material. [Explicit] indicates that parental guidance is recommended to parts of the
    // content." - 5.9
    pub rating: Option<Rating>,
    // "Used to group the podcast into specific sets. May be nested with subcategories like 1. Arts
    // -> 1.1 Design, 1.2 Fashion & Beauty, 1.3[] Food[,] 1.4 Literature etc..." 5.10
    pub categories: Vec<Category>,
    // NOT IMPLEMENTED: 5.13 RSS/CHANNEL/SPOTIFY:LIMIT
    // "Defines the intended market/territory where the podcast is relevant to the consumer...
    // [C]ountry codes ranked in order of priority from most relevant to least relevant. Podcasts
    // with a narrow list of countries will have a higher potential reaching [its]
    // 5.14
    pub country_of_origin: Vec<CountryCode>,
    pub episodes: Vec<Episode>,
}
