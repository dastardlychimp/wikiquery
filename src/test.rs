use crate::requests::Query;

pub mod helpers
{
    use super::*;
    
    pub fn assert_query_contains(query: &mut Query, contains: &[&'static str])
    {

        let request = query.build().unwrap();
        let (parts, _body) = request.into_parts();
        let query_str = parts.uri.query().unwrap();

        for c in contains
        {
            assert!(query_str.contains(c));
        }
    }
}