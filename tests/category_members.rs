use wikiquery;
use wikiquery::requests::Query;

mod helpers;
use helpers::send_successful_query;


mod category_members_tests
{
    use super::*;
    
    #[test]
    fn max_data() {
        let mut query = Query::new();

        query.category_members()
            .cm_title("Category:War")
            .cm_prop("ids")
            .cm_prop("title")
            .cm_prop("sortkey")
            .cm_prop("sortkeyprefix")
            .cm_prop("type")
            .cm_prop("timestamp")
            .cm_limit("5")
            .cm_start_hex_sort_key("55454b3f2f0455294b04393939011101e0c1e0c3dcdcdc")
            .cm_dir("desc");

        let uri = query.uri().unwrap();

        let response = send_successful_query(uri);
        let category_members = response.query.category_members.unwrap();
        let first_member = &category_members[0];

        assert!(response.warnings.is_none());

        assert_eq!(category_members.len(), 5);
        assert_eq!(*first_member.title.as_ref().unwrap(), "World War III".to_string());
        assert!(first_member.page_id.is_some());
        assert!(first_member.ns.is_some());
        assert!(first_member.sort_key.is_some());
        assert!(first_member.sort_key_prefix.is_some());
        assert!(first_member.page_type.is_some());
        assert!(first_member.timestamp.is_some());
    }

    #[test]
    fn warning() {
        let mut query = Query::new();

        query.category_members()
            .cm_title("Category:War")
            .cm_prop("bad_prop");
        
        let uri = query.uri().unwrap();

        let response = send_successful_query(uri);
        
        let warnings = response.warnings
            .unwrap()
            .category_members
            .unwrap()
            .warnings;

        let expected = "Unrecognized value for parameter \"cmprop\": bad_prop.".to_string();
        assert_eq!(warnings, expected);
    }
}
