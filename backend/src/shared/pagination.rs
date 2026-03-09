use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<i64>,
    #[serde(alias = "page_size")]
    pub per_page: Option<i64>,
}

impl PaginationParams {
    pub fn offset(&self) -> i64 {
        let page = self.page.unwrap_or(1).max(1);
        let per_page = self.per_page();
        (page - 1) * per_page
    }

    pub fn per_page(&self) -> i64 {
        self.per_page.unwrap_or(20).clamp(1, 100)
    }
}

/// Extended query parameters for list endpoints with search and filter support.
/// All fields are optional for backward compatibility with existing callers.
#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub page: Option<i64>,
    #[serde(alias = "page_size")]
    pub per_page: Option<i64>,
    /// Free-text search term (ILIKE matched against relevant fields)
    pub search: Option<String>,
    /// Filter by status value (exact match)
    pub status: Option<String>,
    /// Filter by date range start (YYYY-MM-DD)
    pub date_from: Option<String>,
    /// Filter by date range end (YYYY-MM-DD)
    pub date_to: Option<String>,
    /// Filter by material_type (MM), department_id (HR), priority (QM), etc.
    pub filter_type: Option<String>,
}

impl ListParams {
    pub fn offset(&self) -> i64 {
        let page = self.page.unwrap_or(1).max(1);
        let per_page = self.per_page();
        (page - 1) * per_page
    }

    pub fn per_page(&self) -> i64 {
        self.per_page.unwrap_or(20).clamp(1, 100)
    }

    /// Convert to a PaginationParams for backward compatibility.
    pub fn to_pagination(&self) -> PaginationParams {
        PaginationParams {
            page: self.page,
            per_page: self.per_page,
        }
    }

    /// Returns the search term wrapped in '%' for ILIKE queries, if present.
    pub fn search_pattern(&self) -> Option<String> {
        self.search
            .as_ref()
            .filter(|s| !s.trim().is_empty())
            .map(|s| format!("%{}%", s.trim()))
    }
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
    pub total_pages: i64,
}

impl<T: Serialize> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, total: i64, params: &PaginationParams) -> Self {
        let per_page = params.per_page();
        let page = params.page.unwrap_or(1).max(1);
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Self {
            items: data,
            total,
            page,
            page_size: per_page,
            total_pages,
        }
    }

    pub fn from_list_params(data: Vec<T>, total: i64, params: &ListParams) -> Self {
        let per_page = params.per_page();
        let page = params.page.unwrap_or(1).max(1);
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Self {
            items: data,
            total,
            page,
            page_size: per_page,
            total_pages,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offset_calculation() {
        let params = PaginationParams {
            page: Some(3),
            per_page: Some(10),
        };
        assert_eq!(params.offset(), 20);
    }

    #[test]
    fn per_page_clamping() {
        let too_high = PaginationParams {
            page: Some(1),
            per_page: Some(500),
        };
        assert_eq!(too_high.per_page(), 100);

        let too_low = PaginationParams {
            page: Some(1),
            per_page: Some(0),
        };
        assert_eq!(too_low.per_page(), 1);
    }

    #[test]
    fn default_values() {
        let params = PaginationParams {
            page: None,
            per_page: None,
        };
        assert_eq!(params.per_page(), 20);
        assert_eq!(params.offset(), 0);
    }

    #[test]
    fn list_params_offset_calculation() {
        let params = ListParams {
            page: Some(3),
            per_page: Some(10),
            search: None,
            status: None,
            date_from: None,
            date_to: None,
            filter_type: None,
        };
        assert_eq!(params.offset(), 20);
        assert_eq!(params.per_page(), 10);
    }

    #[test]
    fn list_params_search_pattern() {
        let params = ListParams {
            page: None,
            per_page: None,
            search: Some("hello".to_string()),
            status: None,
            date_from: None,
            date_to: None,
            filter_type: None,
        };
        assert_eq!(params.search_pattern(), Some("%hello%".to_string()));

        let empty = ListParams {
            page: None,
            per_page: None,
            search: Some("  ".to_string()),
            status: None,
            date_from: None,
            date_to: None,
            filter_type: None,
        };
        assert_eq!(empty.search_pattern(), None);
    }

    #[test]
    fn list_params_to_pagination() {
        let params = ListParams {
            page: Some(2),
            per_page: Some(25),
            search: Some("test".to_string()),
            status: None,
            date_from: None,
            date_to: None,
            filter_type: None,
        };
        let pag = params.to_pagination();
        assert_eq!(pag.page, Some(2));
        assert_eq!(pag.per_page, Some(25));
    }

    #[test]
    fn paginated_response_total_pages() {
        let params = PaginationParams {
            page: Some(1),
            per_page: Some(10),
        };
        let resp = PaginatedResponse::new(vec![1, 2, 3], 25, &params);
        assert_eq!(resp.total_pages, 3);
        assert_eq!(resp.total, 25);
        assert_eq!(resp.page, 1);
        assert_eq!(resp.page_size, 10);
        assert_eq!(resp.items.len(), 3);
    }

    #[test]
    fn paginated_response_single_page() {
        let params = PaginationParams {
            page: Some(1),
            per_page: Some(50),
        };
        let resp = PaginatedResponse::new(vec!["a", "b"], 2, &params);
        assert_eq!(resp.total_pages, 1);
    }

    #[test]
    fn paginated_response_empty() {
        let params = PaginationParams {
            page: Some(1),
            per_page: Some(20),
        };
        let resp: PaginatedResponse<i32> = PaginatedResponse::new(vec![], 0, &params);
        assert_eq!(resp.total_pages, 0);
        assert_eq!(resp.items.len(), 0);
    }

    #[test]
    fn paginated_response_from_list_params() {
        let params = ListParams {
            page: Some(2),
            per_page: Some(5),
            search: None,
            status: None,
            date_from: None,
            date_to: None,
            filter_type: None,
        };
        let resp = PaginatedResponse::from_list_params(vec![10, 20], 12, &params);
        assert_eq!(resp.page, 2);
        assert_eq!(resp.page_size, 5);
        assert_eq!(resp.total_pages, 3);
    }

    #[test]
    fn list_params_search_pattern_none() {
        let params = ListParams {
            page: None,
            per_page: None,
            search: None,
            status: None,
            date_from: None,
            date_to: None,
            filter_type: None,
        };
        assert_eq!(params.search_pattern(), None);
    }

    #[test]
    fn negative_page_defaults_to_one() {
        let params = PaginationParams {
            page: Some(-5),
            per_page: Some(10),
        };
        // page.max(1) => 1, so offset = 0
        assert_eq!(params.offset(), 0);
    }
}
