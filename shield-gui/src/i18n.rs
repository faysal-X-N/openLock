use sys_locale::get_locale;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Language {
    En,
    Zh,
}

impl Default for Language {
    fn default() -> Self {
        let locale = get_locale().unwrap_or_else(|| "en-US".to_string());
        if locale.starts_with("zh") {
            Language::Zh
        } else {
            Language::En
        }
    }
}

pub struct TextResources {
    // App Title
    pub app_title: &'static str,
    
    // Login
    pub master_password_label: &'static str,
    pub unlock_vault_btn: &'static str,
    pub login_failed_prefix: &'static str,
    
    // Dashboard Top Bar
    pub lock_btn: &'static str,
    pub refresh_btn: &'static str,
    pub check_updates_btn: &'static str,
    pub add_new_btn: &'static str,
    pub search_placeholder: &'static str, // Though egui doesn't strictly have placeholder for singleline, we might use it as label or tooltip
    
    // Entry List
    pub no_entries_found: &'static str,
    
    // Entry Details
    pub select_entry_hint: &'static str,
    pub username_label: &'static str,
    pub password_label: &'static str,
    pub url_label: &'static str,
    pub notes_label: &'static str,
    pub copy_username_tooltip: &'static str,
    pub copy_password_tooltip: &'static str,
    pub edit_btn: &'static str,
    pub delete_btn: &'static str,
    pub confirm_delete: &'static str, // Potential future use
    
    // Edit Form
    pub edit_entry_title: &'static str,
    pub new_entry_title: &'static str,
    pub name_label: &'static str,
    pub save_btn: &'static str,
    pub cancel_btn: &'static str,

    // Language
    pub language_label: &'static str,
}

impl Language {
    pub fn texts(&self) -> TextResources {
        match self {
            Language::En => TextResources {
                app_title: "Shield Password Manager",
                master_password_label: "Master Password:",
                unlock_vault_btn: "Unlock Vault",
                login_failed_prefix: "Login failed: ",
                lock_btn: "Lock",
                refresh_btn: "Refresh",
                check_updates_btn: "Check for Updates",
                add_new_btn: "Add New",
                search_placeholder: "Search...",
                no_entries_found: "No entries found",
                select_entry_hint: "Select an entry to view details",
                username_label: "Username:",
                password_label: "Password:",
                url_label: "URL:",
                notes_label: "Notes:",
                copy_username_tooltip: "Copy Username",
                copy_password_tooltip: "Copy Password",
                edit_btn: "Edit",
                delete_btn: "Delete",
                confirm_delete: "Are you sure?",
                edit_entry_title: "Edit Entry",
                new_entry_title: "New Entry",
                name_label: "Name:",
                save_btn: "Save",
                cancel_btn: "Cancel",
                language_label: "Language / 语言",
            },
            Language::Zh => TextResources {
                app_title: "Shield 密码管理器",
                master_password_label: "主密码：",
                unlock_vault_btn: "解锁密码库",
                login_failed_prefix: "登录失败：",
                lock_btn: "锁定",
                refresh_btn: "刷新",
                check_updates_btn: "检查更新",
                add_new_btn: "新建",
                search_placeholder: "搜索...",
                no_entries_found: "未找到条目",
                select_entry_hint: "选择一个条目以查看详情",
                username_label: "用户名：",
                password_label: "密码：",
                url_label: "网址：",
                notes_label: "备注：",
                copy_username_tooltip: "复制用户名",
                copy_password_tooltip: "复制密码",
                edit_btn: "编辑",
                delete_btn: "删除",
                confirm_delete: "确定删除吗？",
                edit_entry_title: "编辑条目",
                new_entry_title: "新建条目",
                name_label: "名称：",
                save_btn: "保存",
                cancel_btn: "取消",
                language_label: "Language / 语言",
            },
        }
    }
}
