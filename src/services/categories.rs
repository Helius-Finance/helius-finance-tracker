//! Category service: wraps the category-domain methods of [`crate::db::Db`]
//! behind typed request structs.
//!
//! Mirrors the pattern established by [`crate::services::accounts`].

use crate::db::Db;
use crate::error::AppError;
use crate::model::{Category, CategoryKind};

/// Request for [`CategoryService::add`].
#[derive(Clone, Debug)]
pub struct AddCategoryRequest {
    pub name: String,
    pub kind: CategoryKind,
}

/// Request for [`CategoryService::edit`]. At least one of `name` or `kind`
/// must be `Some` — the service enforces that uniformly across frontends.
#[derive(Clone, Debug, Default)]
pub struct EditCategoryRequest {
    pub reference: String,
    pub name: Option<String>,
    pub kind: Option<CategoryKind>,
}

/// Request for [`CategoryService::delete`].
#[derive(Clone, Debug)]
pub struct DeleteCategoryRequest {
    pub reference: String,
}

/// Request for [`CategoryService::list`]. Currently empty; kept as a struct
/// so future filters can be added without breaking callers.
#[derive(Clone, Debug, Default)]
pub struct ListCategoriesRequest;

/// Service facade for the category domain.
pub struct CategoryService<'a> {
    db: &'a Db,
}

impl<'a> CategoryService<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    /// Inserts a new category. Returns the freshly assigned row id.
    pub fn add(&self, req: AddCategoryRequest) -> Result<i64, AppError> {
        self.db.add_category(&req.name, &req.kind)
    }

    /// Updates an existing category. Requires at least one field to change;
    /// frontends should surface the error message verbatim.
    pub fn edit(&self, req: EditCategoryRequest) -> Result<i64, AppError> {
        if req.name.is_none() && req.kind.is_none() {
            return Err(AppError::Validation(
                "category edit requires --name, --kind, or both".to_string(),
            ));
        }
        self.db
            .edit_category(&req.reference, req.name.as_deref(), req.kind.as_ref())
    }

    /// Archives a category (soft-delete). Returns the row id of the archived
    /// category for status messaging.
    pub fn delete(&self, req: DeleteCategoryRequest) -> Result<i64, AppError> {
        self.db.delete_category(&req.reference)
    }

    /// Lists all non-archived categories, ordered by kind then name.
    pub fn list(&self, _req: ListCategoriesRequest) -> Result<Vec<Category>, AppError> {
        self.db.list_categories()
    }
}
