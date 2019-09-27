use wikiquery;
use wikiquery::requests::Query;

mod helpers;
use helpers::send_successful_query;


mod all_categories_tests
{
    use super::*;
    
    #[test]
    fn max_data() {
        let mut query = Query::new();

        query.all_categories()
            .ac_from("War")
            .ac_to("Writing")
            .ac_limit("5")
            .ac_min("1")
            .ac_max("50")
            .ac_prop("size")
            .ac_prop("hidden");

        let uri = query.uri().unwrap();

        let response = send_successful_query(uri);
        let all_categories = response.query.all_categories.unwrap();
        let first_category = &all_categories[0];

        assert!(response.warnings.is_none());

        assert_eq!(all_categories.len(), 5);
        assert_eq!(first_category.category, "War".to_string());
        assert!(first_category.size.unwrap() > 0);
        assert!(first_category.pages.is_some());
        assert!(first_category.files.is_some());
        assert!(first_category.subcats.is_some());
    }

    #[test]
    fn warning() {
        let mut query = Query::new();

        query.all_categories()
            .ac_from("War")
            .ac_prop("bad_prop");
        
        let uri = query.uri().unwrap();

        let response = send_successful_query(uri);
        
        let warnings = response.warnings
            .unwrap()
            .all_categories
            .unwrap()
            .warnings;

        let expected = "Unrecognized value for parameter \"acprop\": bad_prop.".to_string();
        assert_eq!(warnings, expected);
    }
}
