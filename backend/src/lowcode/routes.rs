use axum::routing::{delete, get, post, put};
use axum::Router;

use crate::lowcode::handlers;
use crate::shared::types::AppState;

// NOTE: List Builder, Dashboard Builder, Document Flow, Import/Export,
// and Workflow Actions routes are added at the end of the router chain.

pub fn routes() -> Router<AppState> {
    Router::new()
        // Projects (Admin)
        .route(
            "/projects",
            get(handlers::projects::list_projects).post(handlers::projects::create_project),
        )
        .route(
            "/projects/{id}",
            get(handlers::projects::get_project)
                .put(handlers::projects::update_project)
                .delete(handlers::projects::delete_project),
        )
        // Operations (Developer)
        .route(
            "/operations",
            get(handlers::operations::list_operations).post(handlers::operations::create_operation),
        )
        .route(
            "/operations/{id}",
            get(handlers::operations::get_operation).put(handlers::operations::update_operation),
        )
        // Form Builder (Developer)
        .route(
            "/operations/{id}/form",
            get(handlers::forms::get_form).put(handlers::forms::save_form),
        )
        // Form Fields
        .route(
            "/fields/{id}",
            get(handlers::fields::get_field).delete(handlers::fields::delete_field),
        )
        // Form Execution (User)
        .route("/exec/{code}", get(handlers::executor::get_form_by_code))
        .route(
            "/exec/{code}/list-query",
            get(handlers::executor::list_query),
        )
        .route(
            "/exec/{code}/data",
            get(handlers::executor::list_data).post(handlers::executor::create_data),
        )
        .route(
            "/exec/{code}/data/{id}",
            get(handlers::executor::get_data)
                .put(handlers::executor::update_data)
                .delete(handlers::executor::delete_data),
        )
        // Data Sources
        .route(
            "/datasource/query",
            post(handlers::datasource::execute_query),
        )
        .route("/datasource/tables", get(handlers::datasource::list_tables))
        .route(
            "/datasource/validate-sql",
            post(handlers::datasource::validate_sql),
        )
        .route(
            "/datasource/tables/{table_name}/columns",
            get(handlers::datasource::list_columns),
        )
        // Permissions — Operations
        .route(
            "/permissions/operations/{operation_id}",
            get(handlers::permissions::list_operation_permissions)
                .post(handlers::permissions::create_operation_permission),
        )
        .route(
            "/permissions/operations/{operation_id}/{id}",
            put(handlers::permissions::update_operation_permission)
                .delete(handlers::permissions::delete_operation_permission),
        )
        // Permissions — Fields
        .route(
            "/permissions/fields/{field_id}",
            get(handlers::permissions::list_field_permissions)
                .post(handlers::permissions::create_field_permission),
        )
        .route(
            "/permissions/fields/{field_id}/{id}",
            delete(handlers::permissions::delete_field_permission),
        )
        // Permissions — Record Policies
        .route(
            "/permissions/records/{operation_id}",
            get(handlers::permissions::list_record_policies)
                .post(handlers::permissions::create_record_policy),
        )
        .route(
            "/permissions/records/{operation_id}/{id}",
            delete(handlers::permissions::delete_record_policy),
        )
        // Releases
        .route(
            "/releases",
            get(handlers::releases::list_releases).post(handlers::releases::create_release),
        )
        .route("/releases/{id}", get(handlers::releases::get_release))
        .route(
            "/releases/{id}/submit",
            put(handlers::releases::submit_release),
        )
        .route(
            "/releases/{id}/approve",
            put(handlers::releases::approve_release),
        )
        .route(
            "/releases/{id}/reject",
            put(handlers::releases::reject_release),
        )
        .route(
            "/releases/{id}/publish",
            put(handlers::releases::publish_release),
        )
        // Release Rollback
        .route(
            "/releases/{id}/rollback",
            post(handlers::releases::rollback_release),
        )
        // Feedback
        .route(
            "/feedback",
            get(handlers::feedback::list_feedback).post(handlers::feedback::create_feedback),
        )
        .route("/feedback/{id}", put(handlers::feedback::update_feedback))
        .route(
            "/feedback/{id}/comments",
            get(handlers::feedback::list_comments).post(handlers::feedback::add_comment),
        )
        // Journal
        .route(
            "/operations/{id}/journal",
            get(handlers::journal::list_journal),
        )
        .route(
            "/operations/{id}/journal/rollback/{version}",
            post(handlers::journal::rollback_to_version),
        )
        // Files
        .route("/files/upload", post(handlers::files::upload_file))
        .route("/files/{id}", get(handlers::files::download_file))
        // Navigation
        .route(
            "/navigation",
            get(handlers::navigation::list_navigation)
                .post(handlers::navigation::create_navigation_item),
        )
        .route(
            "/navigation/{id}",
            put(handlers::navigation::update_navigation_item)
                .delete(handlers::navigation::delete_navigation_item),
        )
        .route(
            "/navigation/reorder",
            put(handlers::navigation::reorder_navigation),
        )
        // Notifications
        .route(
            "/notifications",
            get(handlers::notifications::list_notifications),
        )
        .route(
            "/notifications/{id}/read",
            put(handlers::notifications::mark_read),
        )
        .route(
            "/notifications/read-all",
            put(handlers::notifications::mark_all_read),
        )
        // User Profile
        .route("/user/me", get(handlers::user_profile::get_my_profile))
        // Role Management (Admin)
        .route(
            "/users",
            get(handlers::role_management::list_users_with_roles),
        )
        .route(
            "/users/{user_id}/roles",
            post(handlers::role_management::assign_platform_role),
        )
        .route(
            "/users/{user_id}/roles/{role_name}",
            delete(handlers::role_management::revoke_platform_role),
        )
        // Project Developer Management
        .route(
            "/projects/{id}/developers",
            get(handlers::role_management::list_project_developers)
                .post(handlers::role_management::assign_project_developer),
        )
        .route(
            "/projects/{id}/developers/{user_id}",
            delete(handlers::role_management::remove_project_developer),
        )
        // List Builder (Developer)
        .route(
            "/operations/{id}/list",
            get(handlers::list_builder::get_list_definition)
                .put(handlers::list_builder::save_list_definition),
        )
        // Dashboard Builder (Developer)
        .route(
            "/operations/{id}/dashboard",
            get(handlers::dashboard_builder::get_dashboard)
                .put(handlers::dashboard_builder::save_dashboard),
        )
        // Document Flow
        .route(
            "/document-flow/{source_type}/{source_id}",
            get(handlers::document_flow::get_document_flow),
        )
        // Bulk Import
        .route(
            "/exec/{code}/bulk-import",
            post(handlers::import_export::bulk_import),
        )
        // Export
        .route("/exec/{code}/export", get(handlers::export::export_data))
        // Workflow Transition
        .route(
            "/exec/{code}/data/{id}/transition",
            post(handlers::workflow_actions::transition_status),
        )
        // Module Operations (User)
        .route(
            "/modules/{module}/operations",
            get(handlers::module_operations::list_module_operations),
        )
        // Operation Buttons (Developer)
        .route(
            "/operations/{id}/buttons",
            get(handlers::buttons::get_buttons).put(handlers::buttons::save_buttons),
        )
        // Operation Buttons (User — by code)
        .route(
            "/exec/{code}/buttons",
            get(handlers::buttons::get_buttons_by_code),
        )
        // AI Chat (Developer)
        .route("/operations/{id}/ai/chat", post(handlers::ai_chat::chat))
        .route("/ai/status", get(handlers::ai_chat::ai_status))
}
