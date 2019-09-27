use super::{Params, Query, SubQuery};

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
    pub(super) params: &'b mut Params<'a>
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

#[cfg(test)]
mod all_categories_tests
{
    use crate::requests::Query;
    use crate::test::helpers::*;

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
}