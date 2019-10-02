//! Generates [`http`] requests for [`mediawiki`] queries.
//! 
//! Current implementations include:
//! - [`AllCategoriesQuery`]
//! - [`CategoryMembersQuery`]
//! // PagesQuery is only partially implemented.
//! - [`PagesQuery`]
//! 
//! Find documentation for the different queries at [`mediawiki`].
//! 
//! [`mediawiki`]: https://www.mediawiki.org/wiki/API:Query
//! [`PagesQuery`]: pages/struct.PagesQuery.html
//! [`AllCategoriesQuery`]: struct.AllCategoriesQuery.html
//! [`CategoryMembersQuery`]: struct.CategoryMembersQuery.html

use http::{Request, Uri};

use std::collections::HashMap;

use crate::responses;

pub mod all_categories;
pub mod category_members;
pub mod pages;

use all_categories::AllCategoriesQuery;
use category_members::CategoryMembersQuery;
use pages::PagesQuery;

pub type Params<'a> = HashMap<&'a str, String>;

/// A builder to generate mediawiki queries.
/// 
pub struct Query<'a>
{
    pub params: Params<'a>,
}

impl<'a, 'b> Query<'a>
{
    pub fn new() -> Query<'a>
    {
        Query {
            params: HashMap::new()
        }
    }
    
    /// Creates a new [`AllCategoriesQuery`]
    /// 
    /// # Examples
    /// ```
    /// use wikiquery::requests::Query;
    /// 
    /// let mut query = Query::new();
    /// query.all_categories();
    /// query.build().unwrap();
    /// ```
    pub fn all_categories(&'b mut self) -> AllCategoriesQuery<'a, 'b>
    {
        AllCategoriesQuery::new(&mut self.params)
    }

    /// Creates a new [`CategoryMembersQuery`]
    /// 
    /// # Examples
    /// ```
    /// use wikiquery::requests::Query;
    /// 
    /// let mut query = Query::new();
    /// query.category_members();
    /// query.build().unwrap();
    /// ```
    pub fn category_members(&'b mut self) -> CategoryMembersQuery<'a, 'b>
    {
        CategoryMembersQuery::new(&mut self.params)
    }

    /// Creates a new pages query
    /// 
    /// Gets information on specific pages.
    pub fn pages(&'b mut self) -> PagesQuery<'a, 'b>
    {
        PagesQuery::new(&mut self.params)
    }

    /// Add the format param to the query
    /// 
    /// When [`Query::build`] is called, will assign `format=json` by default unless
    /// format was already set.
    pub fn format<S: Into<String>>(&mut self, format: S) -> &mut Self
    {
        self.params.insert("format", format.into());
        self
    }

    /// Generates an [`http`] [`Request`] from the query
    /// 
    /// # Examples
    /// ```
    /// use wikiquery::requests::Query;
    /// 
    /// let mut query = Query::new();
    /// 
    /// query.all_categories()
    ///     .ac_from("Lists_of_colors");
    /// 
    /// let http_request = query.build().unwrap();
    /// ```
    pub fn build(&mut self) -> Result<Request<()>, http::Error>
    {
        let uri = self.uri()?;

        Request::builder()
            .method("GET")
            .uri(uri)
            .body(())
    }

    /// Build a uri for the query
    ///
    /// # Examples
    /// ```
    /// use wikiquery::requests::Query;
    /// 
    /// let mut query = Query::new();
    /// 
    /// query.all_categories()
    ///     .ac_from("Lists_of_colors");
    /// 
    /// let uri = query.uri().unwrap();
    /// ```
    pub fn uri(&mut self) -> Result<Uri, http::Error>
    {
        self.params.entry("format").or_insert("json".to_string());
        self.params.entry("formatversion").or_insert("2".to_string());
        self.params.insert("action", "query".to_string());
        
        let query_string = self.params.iter()
            .fold(
                String::from("/w/api.php?"),
                |acc, (key, value)| format!("{}&{}={}", acc, key, value)
            );

        Uri::builder()
            .scheme("https")
            .authority("en.wikipedia.org")
            .path_and_query(query_string.as_str())
            .build()
    }

    /// Continue a query for more data
    /// 
    /// When a query isn't able to return all the data, you can continue the
    /// query from a [`Query::ContinueBlock`] to receive more.
    /// 
    /// # Examples
    /// ```
    /// use wikiquery::requests::Query;
    /// # use wikiquery::responses::{Query as QueryResponse, QueryBlock, ContinueBlock};
    /// 
    /// let mut query = Query::new();
    /// 
    /// query.all_categories().ac_limit("500");
    /// 
    /// let req = query.build().unwrap();
    /// 
    /// /*
    ///     Send the request and receive a responses::Query
    ///     let resp = _;
    /// */
    /// 
    /// # let resp = QueryResponse {
    /// #     batch_complete: true,
    /// #     query: QueryBlock {all_categories: None, category_members: None, pages: None},
    /// #     warnings: None,
    /// #     continue_block: Some(ContinueBlock {
    /// #         r#continue: String::new(),
    /// #         ac_continue: Some("Archives".to_string()),
    /// #         cm_continue: None,
    /// #         in_continue: None,
    /// #     })
    /// # };
    /// 
    /// query.continue_query(&resp.continue_block);
    /// 
    /// query.build().unwrap();
    /// ```
    /// [`Query::ContinueBlock`]: struct.ContinueBlock.html
    pub fn continue_query(&mut self, continue_block: &Option<responses::ContinueBlock>) -> &Self
    {
        if let Some(continue_block) = continue_block
        {
            self.params.insert("continue", continue_block.r#continue.to_string());

            if let Some(cont) = &continue_block.ac_continue
            {
                self.params.insert("accontinue", cont.to_string());
            }

            if let Some(cont) = &continue_block.cm_continue
            {
                self.params.insert("cmcontinue", cont.to_string());
            }

            if let Some(cont) = &continue_block.in_continue
            {
                self.params.insert("incontinue", cont.to_string());
            }

        }

        self
    }
}

trait SubQuery<'a, 'b> {
    fn get_mut_params(&mut self) -> &mut Params<'a>;
    
    fn add_param_value(&mut self, key: &'a str, val: String) -> &mut Self
    {
        let params = self.get_mut_params();

        if let Some(v) = params.get_mut(key)
        {
            *v = format!("{}|{}", *v, val);
        }
        else
        {
            params.insert(key, val);
        }

        self
    }
}


macro_rules! impl_sub_query
{
    ( $struct:ident ) =>
    {
        impl<'a, 'b> SubQuery<'a, 'b> for $struct<'a, 'b>
        {
            fn get_mut_params(&mut self) -> &mut Params<'a>
            {
                self.params
            }
        }
    }
}

impl_sub_query!(CategoryMembersQuery);
impl_sub_query!(AllCategoriesQuery);
impl_sub_query!(PagesQuery);

#[cfg(test)]
mod test
{
    use super::*;
    use crate::test::helpers::*;

    #[test]
    fn test_combined_requests()
    {
        let mut query = Query::new();

        {
            query.all_categories()
                .ac_min("1")
                .ac_from("Lists_of_colors");
        }

        {
            query.category_members()
                .cm_title("Lists_of_colors")
                .cm_type("page");
        }

        let contains = [
            "list=allcategories|categorymembers",
            "acfrom=Lists_of_colors",
            "acmin=1",
            "cmtype=page",
            "action=query",
            "format=json"
        ];

        assert_query_contains(&mut query, &contains);
    }

    #[test]
    fn test_all_fields_continue_query()
    {
        let mut query = Query::new();

        let continue_block = responses::ContinueBlock
        {
            r#continue: "-||".to_string(),
            ac_continue: Some("a".to_string()),
            cm_continue: Some("b".to_string()),
            in_continue: Some("c".to_string()),
        };

        query.continue_query(&Some(continue_block));

        let contains = [
            "continue=-||",
            "accontinue=a",
            "cmcontinue=b",
            "incontinue=c"
        ];

        assert_query_contains(&mut query, &contains);
    }
}