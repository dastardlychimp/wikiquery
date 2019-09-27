//! Generates [`http`] requests for [`mediawiki`] queries.
//! 
//! Current implementations include:
//! - [`AllCategoriesQuery`]
//! - [`CategoryMembersQuery`]
//! 
//! Find documentation for the different queries at [`mediawiki`].
//! 
//! [`mediawiki`]: https://www.mediawiki.org/wiki/API:Query
//! [`AllCategoriesQuery`]: struct.AllCategoriesQuery.html
//! [`CategoryMembersQuery`]: struct.CategoryMembersQuery.html

use http::{Request, Uri};

use std::collections::HashMap;

use crate::responses;

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
    /// #     query: QueryBlock {all_categories: None, category_members: None},
    /// #     warnings: None,
    /// #     continue_block: Some(ContinueBlock {
    /// #         r#continue: String::new(),
    /// #         ac_continue: Some("Archives".to_string()),
    /// #         cm_continue: None
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

        }

        self
    }
}

/// Generates an *allcategories* list query.
/// 
/// Param documentation can be found at [`mediawiki:allcategories`]
/// 
/// # Examples
/// ```
/// use wikiquery::requests::Query;
/// 
/// let mut query = Query::new();
/// 
/// query.all_categories()
///     .ac_from("Lists_of_colors")
///     .ac_prop("size")
///     .ac_min("1")
///     .ac_limit("5");
/// 
/// let request = query.build().unwrap();
/// ```
/// 
/// [`mediawiki:allcategories`]: https://www.mediawiki.org/wiki/API:Allcategories
pub struct AllCategoriesQuery<'a, 'b>
{
    params: &'b mut Params<'a>
}

#[allow(dead_code)]
impl<'a, 'b> AllCategoriesQuery<'a, 'b>
{
    pub fn new(params: &'b mut Params<'a>) -> AllCategoriesQuery<'a, 'b>
    {
        let mut this = AllCategoriesQuery
        {
            params
        };

        this.add_param_value("list", "allcategories".to_string());

        this
    }

    pub fn ac_from<S: Into<String>>(&mut self, value: S) -> &mut Self
    {
        self.add_param_value("acfrom", value.into())
    }

    pub fn ac_to<S: Into<String>>(&mut self, value: S) -> &mut Self
    {
        self.add_param_value("acto", value.into())
    }

    pub fn ac_prop<S: Into<String>>(&mut self, value: S) -> &mut Self
    {
        self.add_param_value("acprop", value.into())
    }

    pub fn ac_min<S: Into<String>>(&mut self, value: S) -> &mut Self
    {
        self.add_param_value("acmin", value.into())
    }

    pub fn ac_max<S: Into<String>>(&mut self, value: S) -> &mut Self
    {
        self.add_param_value("acmax", value.into())
    }

    pub fn ac_limit<S: Into<String>>(&mut self, value: S) -> &mut Self
    {
        self.add_param_value("aclimit", value.into())
    }
    
    pub fn ac_prefix<S: Into<String>>(&mut self, value: S) -> &mut Self
    {
        self.add_param_value("acprefix", value.into())
    }

    pub fn ac_dir<S: Into<String>>(&mut self, value: S) -> &mut Self
    {
        self.add_param_value("acdir", value.into())
    }

    pub fn ac_continue<S: Into<String>>(&mut self, value: S) -> &mut Self
    {
        self.add_param_value("accontinue", value.into())
    }
}

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
    params: &'b mut Params<'a>
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

impl_sub_query!(AllCategoriesQuery);
impl_sub_query!(CategoryMembersQuery);


#[cfg(test)]
mod test
{
    use super::*;

    fn assert_query_contains(query: &mut Query, contains: &[&'static str])
    {

        let request = query.build().unwrap();
        let (parts, _body) = request.into_parts();
        let query_str = parts.uri.query().unwrap();

        for c in contains
        {
            assert!(query_str.contains(c));
        }
    }

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
        };

        query.continue_query(&Some(continue_block));

        let contains = ["continue=-||", "accontinue=a", "cmcontinue=b"];
        assert_query_contains(&mut query, &contains);
    }

    #[test]
    fn test_all_fields_all_categories()
    {
        let mut query = Query::new();

        query.all_categories()
            .ac_from("1")
            .ac_to("2")
            .ac_prop("3")
            .ac_min("4")
            .ac_max("5")
            .ac_limit("6")
            .ac_prefix("7")
            .ac_dir("8")
            .ac_continue("9");

        let contains = [
            "acfrom=1",
            "acto=2",
            "acprop=3",
            "acmin=4",
            "acmax=5",
            "aclimit=6",
            "acprefix=7",
            "acdir=8",
            "accontinue=9",
        ];

        assert_query_contains(&mut query, &contains);
    }

    #[test]
    fn test_all_fields_category_members() {
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