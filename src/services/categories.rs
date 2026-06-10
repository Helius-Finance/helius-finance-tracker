use crate::db::Db;
use crate::error::AppError;
use crate::model::{Category, CategoryKind};

#[derive(Clone, Debug)]
pub struct AddCategoryRequest {
    pub name: String,
    pub kind: CategoryKind,
}

#[derive(Clone, Debug, Default)]
pub struct EditCategoryRequest {
    pub reference: String,
    pub name: Option<String>,
    pub kind: Option<CategoryKind>,
}

#[derive(Clone, Debug)]
pub struct DeleteCategoryRequest {
    pub reference: String,
}

#[derive(Clone, Debug, Default)]
pub struct ListCategoriesRequest;

pub struct CategoryService<'a> {
    db: &'a Db,
}

impl<'a> CategoryService<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    pub fn add(&self, req: AddCategoryRequest) -> Result<i64, AppError> {
        self.db.add_category(&req.name, &req.kind)
    }

    pub fn edit(&self, req: EditCategoryRequest) -> Result<i64, AppError> {
        if req.name.is_none() && req.kind.is_none() {
            return Err(AppError::Validation(
                "category edit requires --name, --kind, or both".to_string(),
            ));
        }
        self.db
            .edit_category(&req.reference, req.name.as_deref(), req.kind.as_ref())
    }

    pub fn delete(&self, req: DeleteCategoryRequest) -> Result<i64, AppError> {
        self.db.delete_category(&req.reference)
    }

    pub fn list(&self, _req: ListCategoriesRequest) -> Result<Vec<Category>, AppError> {
        self.db.list_categories()
    }
}
