use serde;
use serde::{Deserialize};

use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ContinueBlock
{
    pub r#continue: String,
    #[serde(rename="accontinue")]
    pub ac_continue: Option<String>,
    #[serde(rename="cmcontinue")]
    pub cm_continue: Option<String>,
    #[serde(rename="incontinue")]
    pub in_continue: Option<String>,
    #[serde(rename="desccontinue")]
    pub desc_continue: Option<String>,
    #[serde(rename="excontinue")]
    pub ex_continue: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct QueryBlock
{
    pub pages: Option<Vec<pages::Data>>,
    #[serde(rename="allcategories")]
    pub all_categories: Option<Vec<all_categories::Data>>,
    #[serde(rename="categorymembers")]
    pub category_members: Option<Vec<category_members::Data>>
}

#[derive(Debug, Deserialize)]
pub struct WarningBlock
{
    #[serde(rename="allcategories")]
    pub all_categories: Option<Warnings>,
    #[serde(rename="categorymembers")]
    pub category_members: Option<Warnings>,
    pub info: Option<Warnings>,
    pub pages: Option<Warnings>,
    pub description: Option<Warnings>,
    pub extracts: Option<Warnings>,
}

#[derive(Debug, Deserialize)]
pub struct Query
{
    #[serde(rename = "batchcomplete")]
    pub batch_complete: bool,
    pub query: QueryBlock,
    #[serde(rename = "continue")]
    pub continue_block: Option<ContinueBlock>,
    pub warnings: Option<WarningBlock>,
}


#[derive(Debug, Deserialize)]
pub struct Warnings
{
    pub warnings: String,
}

pub mod all_categories
{
    use super::*;
    
    #[derive(Debug, Deserialize)]
    pub struct Data
    {
        pub category: String,
        pub size: Option<u32>,
        pub pages: Option<u32>,
        pub files: Option<u32>,
        pub subcats: Option<u32>,
    }
}

pub mod category_members
{
    use super::*;
    
    #[derive(Debug, Deserialize)]
    pub struct Data
    {
        #[serde(rename="pageid")]
        pub page_id: Option<u32>,
        pub ns: Option<u32>,
        #[serde(rename="sortkey")]
        pub sort_key: Option<String>,
        #[serde(rename="sortkeyprefix")]
        pub sort_key_prefix: Option<String>,
        pub title: Option<String>,
        #[serde(rename="type")]
        pub page_type: Option<String>,
        pub timestamp: Option<String>,
    }
}

pub mod pages
{
    use super::*;

    #[derive(Debug, Deserialize)]
    pub struct Data
    {
        // Default data
        pub ns: u32,
        pub title: String,
        pub missing: Option<bool>,
        #[serde(rename="pageid")]
        pub page_id: u64,

        // -----
        // Data from the description prop
        // -----
        pub description: Option<String>,
        #[serde(rename="descriptionsource")]
        pub description_source: Option<String>,

        // -----
        // Data from the extracts prop
        // -----
        pub extract: Option<String>,

        // -----
        // Data from the info prop
        // -----
        #[serde(rename="contentmodel")]
        pub content_model: Option<String>,
        #[serde(rename="pagelanguage")]
        pub page_language: Option<String>,
        #[serde(rename="pagelanguagehtmlcode")]
        pub page_language_html_code: Option<String>,
        #[serde(rename="pagelanguagedir")]
        pub page_language_dir: Option<String>,
        pub touched: Option<String>,
        #[serde(rename="lastrevid")]
        pub last_rev_id: Option<u32>,
        pub length: Option<u32>,
        pub protection: Option<Vec<info::Protection>>,
        #[serde(rename="restrictiontypes")]
        pub restriction_types: Option<Vec<String>>,
        #[serde(rename="fullurl")]
        pub full_url: Option<String>,
        #[serde(rename="editurl")]
        pub edit_url: Option<String>,
        #[serde(rename="canonicalurl")]
        pub canonical_url: Option<String>,
        #[serde(rename="displaytitle")]
        pub display_title: Option<String>,
        pub actions: Option<HashMap<String, Vec<info::Actions>>>,
    }

    pub mod info
    {
        use super::*;
        
        #[derive(Debug, Deserialize)]
        #[serde(untagged)]
        pub enum Actions
        {
            Bool(bool),
            Detailed(DetailedActions),
        }

        #[derive(Debug, Deserialize)]
        pub struct DetailedActions
        {
            code: String,
            text: String,
        }

        #[derive(Debug, Deserialize)]
        pub struct Protection
        {
            #[serde(rename="type")]
            protection_type: String,
            level: String,
            expiry: String,
        }
    }
}

#[cfg(test)]
mod test
{
    use serde_json;
    use super::Query;
    
    #[test]
    fn test_deserialize_all_categories_response() {
        let resp = "{\"batchcomplete\":true,\"continue\":{\"accontinue\":\"Lists_and_galleries_of_flags\",\"continue\":\"-||\"},\"query\":{\"allcategories\":[{\"category\":\"Lists\",\"size\":29,\"pages\":1,\"files\":0,\"subcats\":28},{\"category\":\"Lists American animated television series episode\",\"size\":1,\"pages\":1,\"files\":0,\"subcats\":0},{\"category\":\"Lists American animated television series episodes\",\"size\":1,\"pages\":1,\"files\":0,\"subcats\":0},{\"category\":\"Lists about Wikipedia\",\"size\":6,\"pages\":6,\"files\":0,\"subcats\":0},{\"category\":\"Lists about role-playing games\",\"size\":38,\"pages\":37,\"files\":0,\"subcats\":1}]}}";
        let query: Query = serde_json::from_str(&resp).unwrap();

        assert!(query.query.all_categories.is_some());
    }
    
    #[test]
    fn test_deserialize_category_members_response() {
        let resp = "{\"batchcomplete\":true,\"query\":{\"categorymembers\":[{\"pageid\":37703894,\"ns\":0,\"title\":\"Lists of colors\",\"sortkey\":\"0403063f394d4f4d044533042d453f454b4d011501c4dc12\",\"sortkeyprefix\":\" \",\"type\":\"page\",\"timestamp\":\"2019-01-30T18:32:56Z\"},{\"pageid\":16009151,\"ns\":0,\"title\":\"List of colors (compact)\",\"sortkey\":\"082d454147292d4f03063f394d4f044533042d453f454b4d04098c2d454147292d4f098e012501bddc1b\",\"sortkeyprefix\":\".compact\",\"type\":\"page\",\"timestamp\":\"2016-11-03T04:30:16Z\"},{\"pageid\":435914,\"ns\":0,\"title\":\"List of colors: A–F\",\"sortkey\":\"2d0f0303063f394d4f044533042d453f454b4d072c0429051833011a01dcc4dcb7dcc5dc\",\"sortkeyprefix\":\"C01\",\"type\":\"page\",\"timestamp\":\"2018-11-01T14:19:47Z\"},{\"pageid\":39753811,\"ns\":0,\"title\":\"List of colors: G–M\",\"sortkey\":\"2d0f0403063f394d4f044533042d453f454b4d072c0435051841011a01dcc4dcb7dcc5dc\",\"sortkeyprefix\":\"C02\",\"type\":\"page\",\"timestamp\":\"2016-11-03T04:14:24Z\"},{\"pageid\":37703435,\"ns\":0,\"title\":\"List of colors: N–Z\",\"sortkey\":\"2d0f0503063f394d4f044533042d453f454b4d072c044305185b011a01dcc4dcb7dcc5dc\",\"sortkeyprefix\":\"C03\",\"type\":\"page\",\"timestamp\":\"2016-11-03T04:15:06Z\"},{\"pageid\":50777971,\"ns\":0,\"title\":\"List of colors by shade\",\"sortkey\":\"2d453f454b4d06044d37292f3103063f394d4f044533042d453f454b4d042b59044d37292f3101250801b8dc1a\",\"sortkeyprefix\":\"colors, shade\",\"type\":\"page\",\"timestamp\":\"2018-05-03T06:59:44Z\"},{\"pageid\":3364578,\"ns\":0,\"title\":\"List of Crayola crayon colors\",\"sortkey\":\"2d4b2959453f29042d4b2959454303063f394d4f044533042d4b2959453f29042d4b29594543042d453f454b4d01250f01dcb8dcbfdc18\",\"sortkeyprefix\":\"Crayola crayon\",\"type\":\"page\",\"timestamp\":\"2019-05-06T13:55:08Z\"},{\"pageid\":38181580,\"ns\":0,\"title\":\"List of fictional colors\",\"sortkey\":\"33392d4f394543293f03063f394d4f0445330433392d4f394543293f042d453f454b4d01250501bcdc1b\",\"sortkeyprefix\":\"fictional\",\"type\":\"page\",\"timestamp\":\"2017-04-25T21:21:13Z\"},{\"pageid\":2366759,\"ns\":0,\"title\":\"List of Game Boy colors and styles\",\"sortkey\":\"35294131042b455903063f394d4f0445330435294131042b4559042d453f454b4d0429432f044d4f593f314d01250e01dcc2dcc3dcbfdcc2dc18\",\"sortkeyprefix\":\"Game Boy\",\"type\":\"page\",\"timestamp\":\"2016-11-03T02:55:58Z\"},{\"pageid\":1218748,\"ns\":0,\"title\":\"List of international auto racing colours\",\"sortkey\":\"39434f314b43294f394543293f0429514f45044b292d39433503063f394d4f0445330439434f314b43294f394543293f0429514f45044b292d394335042d453f45514b4d0125250501acdc2c\",\"sortkeyprefix\":\"international auto racing\",\"type\":\"page\",\"timestamp\":\"2016-11-03T02:55:09Z\"},{\"pageid\":34437296,\"ns\":0,\"title\":\"List of chicken colours\",\"sortkey\":\"3f394d4f044533042d37392d3d3143042d453f45514b4d011b01dc1a\",\"sortkeyprefix\":\"\",\"type\":\"page\",\"timestamp\":\"2014-11-15T22:12:57Z\"},{\"pageid\":51248620,\"ns\":0,\"title\":\"List of Citadel paints\",\"sortkey\":\"3f394d4f044533042d394f292f313f04472939434f4d011a01dcbfdc11\",\"sortkeyprefix\":\"\",\"type\":\"page\",\"timestamp\":\"2016-11-03T03:18:34Z\"},{\"pageid\":40432339,\"ns\":0,\"title\":\"List of Nintendo 3DS colors and styles\",\"sortkey\":\"4339434f31432f45040f052f4d03063f394d4f044533044339434f31432f45040f052f4d042d453f454b4d0429432f044d4f593f314d01251601dcbadcbfdcbddcdc16\",\"sortkeyprefix\":\"Nintendo 3ds\",\"type\":\"page\",\"timestamp\":\"2014-11-15T22:14:42Z\"},{\"pageid\":7475661,\"ns\":0,\"title\":\"List of Nintendo DS colors and styles\",\"sortkey\":\"4339434f31432f45042f4d03063f394d4f044533044339434f31432f45042f4d042d453f454b4d0429432f044d4f593f314d01251401dcbedcdcc5dcbfdcbedcdc16\",\"sortkeyprefix\":\"Nintendo DS\",\"type\":\"page\",\"timestamp\":\"2016-11-03T02:53:57Z\"},{\"pageid\":4828383,\"ns\":0,\"title\":\"List of color palettes\",\"sortkey\":\"47293f314f4f314d03063f394d4f044533042d453f454b0447293f314f4f314d012301dcbedc19\",\"sortkeyprefix\":\"Palettes\",\"type\":\"page\",\"timestamp\":\"2016-11-03T03:56:48Z\"},{\"pageid\":39425965,\"ns\":0,\"title\":\"List of RAL colors\",\"sortkey\":\"4b293f03063f394d4f044533044b293f042d453f454b4d011a01dcc3dcbfdcdcdc0b\",\"sortkeyprefix\":\"Ral\",\"type\":\"page\",\"timestamp\":\"2016-11-03T03:53:28Z\"},{\"pageid\":1109151,\"ns\":0,\"title\":\"List of U.S. state colors\",\"sortkey\":\"51084d08044d4f294f31042d453f454b4d03063f394d4f0445330451084d08044d4f294f31042d453f454b4d01250e01dcc5dcb7dcbfdcc5dc12\",\"sortkeyprefix\":\"U.S. state colors\",\"type\":\"page\",\"timestamp\":\"2016-11-03T03:51:18Z\"},{\"pageid\":44692823,\"ns\":0,\"title\":\"List of colors in various languages\",\"sortkey\":\"53294b3945514d043f294335512935314d03063f394d4f044533042d453f454b4d0439430453294b3945514d043f294335512935314d01251801b4dc26\",\"sortkeyprefix\":\"various languages\",\"type\":\"page\",\"timestamp\":\"2016-11-03T04:26:11Z\"}]}}";
        let query: Query = serde_json::from_str(&resp).unwrap();

        assert!(query.query.category_members.is_some())
    }

    #[test]
    fn test_deserialize_response_with_warnings() {
        let resp = "{\"batchcomplete\":true,\"warnings\":{\"categorymembers\":{\"warnings\":\"Unrecognized value for parameter \\\"cmprop\\\": I_am_bad_prop.\\nUnrecognized value for parameter \\\"cmtype\\\": I_am_bad_type.\"}},\"query\":{\"categorymembers\":[]}}";
        let query: Query = serde_json::from_str(&resp).unwrap();

        assert!(query.warnings.unwrap().category_members.is_some());
    }
}