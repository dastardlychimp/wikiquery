use wikiquery;
use wikiquery::requests::Query;

mod helpers;
use helpers::send_successful_query;


mod pages_tests
{
    use super::*;

    #[test]
    fn all_info_test() {
        let mut query = Query::new();

        query.pages()
            .titles("Main%20page")
            .info()
            .in_prop("protection")
            .in_prop("talkid")
            .in_prop("watched")
            .in_prop("watchers")
            .in_prop("visitingwatchers")
            .in_prop("notificationtimestamp")
            .in_prop("subjectid")
            .in_prop("url")
            .in_prop("preload")
            .in_prop("displaytitle")
            .in_prop("varianttitles")
            .in_test_actions("protection")
            .in_test_actions("talkid")
            .in_test_actions("watched")
            .in_test_actions("watchers")
            .in_test_actions("visitingwatchers")
            .in_test_actions("notificationtimestamp")
            .in_test_actions("subjectid")
            .in_test_actions("url")
            .in_test_actions("read")
            .in_test_actions("preload")
            .in_test_actions("displaytitle")
            .in_test_actions("varianttitles")
            .in_test_actions_detail("quick");
        
        let uri = query.uri().unwrap();

        let response = send_successful_query(uri);

        let pages = response.query.pages.unwrap();
        let first_page = &pages[0];

        println!("pages: {:?}", &pages);

        assert!(response.warnings.is_none());

        assert_eq!(first_page.ns, 0);
        assert_eq!(first_page.page_id, 217225);
        assert_eq!(first_page.title, "Main page".to_string());
        assert!(first_page.missing.is_none());

        assert!(first_page.content_model.is_some());
        assert!(first_page.page_language.is_some());
        assert!(first_page.page_language_html_code.is_some());
        assert!(first_page.page_language_dir.is_some());
        assert!(first_page.touched.is_some());
        assert!(first_page.last_rev_id.is_some());
        assert!(first_page.length.is_some());
        assert!(first_page.protection.is_some());
        assert!(first_page.restriction_types.is_some());
        assert!(first_page.full_url.is_some());
        assert!(first_page.edit_url.is_some());
        assert!(first_page.canonical_url.is_some());
        assert!(first_page.display_title.is_some());
        assert!(first_page.actions.is_some());
    }
}
