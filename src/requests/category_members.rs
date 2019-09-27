use super::{Params, Query, SubQuery};

/// Generates a *categorymembers* list query.
/// 
/// Param documentation can be found at [`mediawiki:categorymembers`]
/// 
/// # Examples
/// ```
/// use wikiquery::requests::Query;
/// 
/// let mut query = Query::new();
/// 
/// query.category_members()
///     .cm_title("Category:Lists_of_colors")
///     .cm_prop("ids")
///     .cm_prop("size")
///     .cm_prop("type")
///     .cm_prop("timestamp")
///     .cm_type("page")
///     .cm_limit("100");
/// 
/// let request = query.build().unwrap();
/// ```
/// 
/// [`mediawiki:categorymembers`]: https://www.mediawiki.org/wiki/API:Categorymembers
pub struct CategoryMembersQuery<'a, 'b>
{
    pub(super) params: &'b mut Params<'a>
}

#[allow(dead_code)]
impl<'a, 'b> CategoryMembersQuery<'a, 'b>
{
    pub fn new(params: &'b mut Params<'a>) -> CategoryMembersQuery<'a, 'b>
    {
        let mut this = CategoryMembersQuery
        {
            params
        };

        this.add_param_value("list", "categorymembers".to_string());

        this
    }

    pub fn cm_title<S: Into<String>>(&mut self, value: S) -> &mut Self
    {
        self.add_param_value("cmtitle", value.into())
    }

    pub fn cm_page_id<S: Into<String>>(&mut self, value: S) -> &mut Self
    {
        self.add_param_value("cmpageid", value.into())
    }

    pub fn cm_prop<S: Into<String>>(&mut self, value: S) -> &mut Self
    {
        self.add_param_value("cmprop", value.into())
    }

    pub fn cm_type<S: Into<String>>(&mut self, value: S) -> &mut Self
    {
        self.add_param_value("cmtype", value.into())
    }

    pub fn cm_limit<S: Into<String>>(&mut self, value: S) -> &mut Self
    {
        self.add_param_value("cmlimit", value.into())
    }

    pub fn cm_sort<S: Into<String>>(&mut self, value: S) -> &mut Self
    {
        self.add_param_value("cmsort", value.into())
    }

    pub fn cm_dir<S: Into<String>>(&mut self, value: S) -> &mut Self
    {
        self.add_param_value("cmdir", value.into())
    }

    pub fn cm_start<S: Into<String>>(&mut self, value: S) -> &mut Self
    {
        self.add_param_value("cmstart", value.into())
    }

    pub fn cm_end<S: Into<String>>(&mut self, value: S) -> &mut Self
    {
        self.add_param_value("cmend", value.into())
    }

    pub fn cm_start_hex_sort_key<S: Into<String>>(&mut self, value: S) -> &mut Self
    {
        self.add_param_value("cmstarthexsortkey", value.into())
    }

    pub fn cm_end_hex_sort_key<S: Into<String>>(&mut self, value: S) -> &mut Self
    {
        self.add_param_value("cmendhexsortkey", value.into())
    }

    pub fn cm_start_sort_key_prefix<S: Into<String>>(&mut self, value: S) -> &mut Self
    {
        self.add_param_value("cmstartsortkeyprefix", value.into())
    }

    pub fn cm_end_sort_key_prefix<S: Into<String>>(&mut self, value: S) -> &mut Self
    {
        self.add_param_value("cmendsortkeyprefix", value.into())
    }

    pub fn cm_continue<S: Into<String>>(&mut self, value: S) -> &mut Self
    {
        self.add_param_value("cmcontinue", value.into())
    }
}

#[cfg(test)]
mod category_members_tests
{
    use crate::requests::Query;
    use crate::test::helpers::*;
    
    #[test]
    fn all_fields() {
        let mut query = Query::new();

        query.category_members()
            .cm_title("1")
            .cm_page_id("2")
            .cm_prop("3")
            .cm_type("4")
            .cm_limit("5")
            .cm_sort("6")
            .cm_dir("7")
            .cm_start("8")
            .cm_end("9")
            .cm_start_hex_sort_key("10")
            .cm_end_hex_sort_key("11")
            .cm_start_sort_key_prefix("12")
            .cm_end_sort_key_prefix("13")
            .cm_continue("14");

        let contains = [
            "cmtitle=1",
            "cmpageid=2",
            "cmprop=3",
            "cmtype=4",
            "cmlimit=5",
            "cmsort=6",
            "cmdir=7",
            "cmstart=8",
            "cmend=9",
            "cmstarthexsortkey=10",
            "cmendhexsortkey=11",
            "cmstartsortkeyprefix=12",
            "cmendsortkeyprefix=13",
            "cmcontinue=14",
        ];

        assert_query_contains(&mut query, &contains);
    }
}