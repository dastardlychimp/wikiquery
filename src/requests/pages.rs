use super::{Params, SubQuery};


/// Generates a pages query
/// 
/// Param documentation can be found at [`mediawiki:Api`]
/// 
/// Call either the [`PagesQuery::titles`] method or the [`PagesQuery::page_ids`] method to specify
/// the pages. Then call prop methods.
/// 
/// ## Prop methods
/// - [`PagesQuery::info`]
/// 
/// [`PagesQuery::info`]: PagesQuery::info
/// [`PagesQuery::titles`]: PagesQuery::titles
/// [`mediawiki:Api`]: https://www.mediawiki.org/wiki/API
pub struct PagesQuery<'a, 'b>
{
    pub(super) params: &'b mut Params<'a>
}

impl<'a, 'b> PagesQuery<'a, 'b>
{
    pub fn new(params: &'b mut Params<'a>) -> PagesQuery<'a, 'b>
    {
        PagesQuery
        {
            params
        }
    }


    pub fn titles<S: Into<String>>(&mut self, value: S) -> &mut Self
    {
        self.add_param_value("titles", value.into())
    }

    /*
        -----
        Info Query methods
        -----
    */

    /// Adds the info prop
    /// 
    /// Param documentatin can be found at [`mediawiki:Info`]
    /// 
    /// # Examples
    /// ```
    /// use wikiquery::requests::Query;
    /// 
    /// let mut query = Query::new();
    /// 
    /// query.pages()
    ///     .titles("United%20States")
    ///     .info()
    ///     .in_prop("url")
    ///     .in_prop("displaytitle")
    ///     .in_prop("varianttitles")
    ///     .in_prop("subjectid")
    ///     .in_test_actions("read")
    ///     .in_test_actions("url")
    ///     .in_test_actions_detail("bool");
    /// 
    /// let request = query.build().unwrap();
    /// ```
    /// 
    /// [`mediawiki:Info`]: https://www.mediawiki.org/wiki/API:Info
    pub fn info(&'b mut self) -> &mut Self
    {
        self.add_param_value("prop", "info".to_string())
    }

    pub fn in_prop<S: Into<String>>(&mut self, value: S) -> &mut Self
    {
        self.add_param_value("inprop", value.into())
    }

    pub fn in_test_actions<S: Into<String>>(&mut self, value: S) -> &mut Self
    {
        self.add_param_value("intestactions", value.into())
    }

    pub fn in_test_actions_detail<S: Into<String>>(&mut self, value: S) -> &mut Self
    {
        self.add_param_value("intestactionsdetail", value.into())
    }

    pub fn in_continue<S: Into<String>>(&mut self, value: S) -> &mut Self
    {
        self.add_param_value("incontinue", value.into())
    }


    /*
        -----
        Description Query methods
        -----
    */

    /// Adds the description prop
    /// 
    /// Param documentatin can be found at [`mediawiki:Description`]
    /// 
    /// # Examples
    /// ```
    /// use wikiquery::requests::Query;
    /// 
    /// let mut query = Query::new();
    /// 
    /// query.pages()
    ///     .titles("United%20States")
    ///     .description();
    /// 
    /// let request = query.build().unwrap();
    /// ```
    /// 
    /// [`mediawiki:Description`]: https://www.mediawiki.org/wiki/API:Description
    pub fn description(&'b mut self) -> &mut Self
    {
        self.add_param_value("prop", "description".to_string())
    }

    pub fn desc_continue<S: Into<String>>(&'b mut self, value: S) -> &mut Self
    {
        self.add_param_value("desccontinue", value.into())
    }
    
    pub fn desc_prefer_source<S: Into<String>>(&mut self, value: S) -> &mut Self
    {
        self.add_param_value("descprefersource", value.into())
    }

}


#[cfg(test)]
mod pages_tests
{
    use crate::requests::Query;
    use crate::test::helpers::*;
    
    #[test]
    fn info_all_fields() {
        let mut query = Query::new();

        query.pages()
            .titles("1")
            .info()
            .in_prop("2")
            .in_test_actions_detail("3")
            .in_continue("4");

        let contains = [
            "titles=1",
            "prop=info",
            "inprop=2",
            "intestactionsdetail=3",
            "incontinue=4",
        ];

        assert_query_contains(&mut query, &contains);
    }

    #[test]
    fn description_all_fields() {
        let mut query = Query::new();

        query.pages()
            .titles("1")
            .description()
            .desc_prefer_source("1")
            .desc_continue("2");

        let contains = [
            "titles=1",
            "prop=description",
            "descprefersource=1",
            "desccontinue=2"
        ];

        assert_query_contains(&mut query, &contains);
    }
}