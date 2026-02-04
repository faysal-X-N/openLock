#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use secrecy::{SecretBox, ExposeSecret};
use shield_core::{Vault, model::Entry};
use std::path::PathBuf;
use anyhow::{Result, Context};
use copypasta::{ClipboardContext, ClipboardProvider};

mod i18n;
use i18n::{Language, TextResources};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([600.0, 400.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Shield Password Manager",
        options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            Box::new(ShieldApp::default())
        }),
    )
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // 尝试加载 Windows 系统中文字体
    // 优先使用微软雅黑 (msyh.ttc)，其次使用黑体 (simhei.ttf)
    let font_paths = [
        "C:\\Windows\\Fonts\\msyh.ttc",
        "C:\\Windows\\Fonts\\simhei.ttf",
        "C:\\Windows\\Fonts\\simsun.ttc",
    ];

    let mut font_data = Vec::new();
    for path in font_paths {
        if let Ok(data) = std::fs::read(path) {
            font_data = data;
            break;
        }
    }

    if !font_data.is_empty() {
        fonts.font_data.insert(
            "system_chinese".to_owned(),
            egui::FontData::from_owned(font_data),
        );

        // 将中文字体添加到 Proportional (比例字体) 的首位
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "system_chinese".to_owned());

        // 将中文字体添加到 Monospace (等宽字体) 的末尾作为备选
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push("system_chinese".to_owned());

        ctx.set_fonts(fonts);
    } else {
        eprintln!("Warning: No Chinese system fonts found.");
    }
}

struct ShieldApp {
    vault: Option<Vault>,
    entries: Vec<Entry>,
    // Login State
    master_password: String,
    login_error: Option<String>,
    // View State
    search_query: String,
    selected_entry_uuid: Option<uuid::Uuid>,
    language: Language,
    // Editing State
    is_editing: bool,
    edit_entry_name: String,
    edit_entry_username: String,
    edit_entry_password: String,
    edit_entry_url: String,
    edit_entry_notes: String,
}

impl Default for ShieldApp {
    fn default() -> Self {
        Self {
            vault: None,
            entries: Vec::new(),
            master_password: String::new(),
            login_error: None,
            search_query: String::new(),
            selected_entry_uuid: None,
            language: Language::default(),
            is_editing: false,
            edit_entry_name: String::new(),
            edit_entry_username: String::new(),
            edit_entry_password: String::new(),
            edit_entry_url: String::new(),
            edit_entry_notes: String::new(),
        }
    }
}

impl ShieldApp {
    fn try_login(&mut self) {
        let db_path = dirs::data_local_dir()
            .unwrap_or(PathBuf::from("."))
            .join("shield.db");
            
        let password = SecretBox::new(Box::new(self.master_password.clone()));
        
        match Vault::open(&db_path, &password) {
            Ok(vault) => {
                self.vault = Some(vault);
                self.refresh_entries();
                self.login_error = None;
            }
            Err(e) => {
                self.login_error = Some(format!("Login failed: {}", e));
            }
        }
    }

    fn refresh_entries(&mut self) {
        if let Some(vault) = &self.vault {
            if let Ok(entries) = vault.list_entries() {
                self.entries = entries;
            }
        }
    }

    fn text(&self) -> TextResources {
        self.language.texts()
    }

    fn copy_to_clipboard(&self, text: &str) {
        if let Ok(mut ctx) = ClipboardContext::new() {
            let _ = ctx.set_contents(text.to_owned());
        }
    }
}

impl eframe::App for ShieldApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.vault.is_none() {
            self.show_login(ctx);
        } else {
            self.show_dashboard(ctx);
        }
    }
}

impl ShieldApp {
    fn show_login(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.vertical_centered(|ui| {
                let texts = self.text();
                ui.add_space(50.0);
                ui.heading(texts.app_title);
                ui.add_space(10.0);
                
                // Explicit Language Selection
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center).with_main_justify(true), |ui| {
                         ui.label(format!("🌐 {}", texts.language_label));
                         if ui.selectable_label(self.language == Language::En, "🇺🇸 English").clicked() {
                             self.language = Language::En;
                         }
                         if ui.selectable_label(self.language == Language::Zh, "🇨🇳 中文").clicked() {
                             self.language = Language::Zh;
                         }
                    });
                });
                
                ui.add_space(20.0);
                
                ui.label(texts.master_password_label);
                let response = ui.add(egui::TextEdit::singleline(&mut self.master_password).password(true));
                
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.try_login();
                }

                ui.add_space(10.0);
                if ui.button(texts.unlock_vault_btn).clicked() {
                    self.try_login();
                }

                if let Some(err) = &self.login_error {
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                         ui.colored_label(egui::Color32::RED, texts.login_failed_prefix);
                         ui.colored_label(egui::Color32::RED, err);
                    });
                }
            });
        });
    }

    fn show_dashboard(&mut self, ctx: &egui::Context) {
        let texts = self.text();
        
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("🔍");
                ui.add(egui::TextEdit::singleline(&mut self.search_query).hint_text(texts.search_placeholder));
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Language Switcher
                    egui::ComboBox::from_id_source("lang_combo_dash")
                        .selected_text(match self.language {
                            Language::En => "🇺🇸",
                            Language::Zh => "🇨🇳",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.language, Language::En, "🇺🇸 English");
                            ui.selectable_value(&mut self.language, Language::Zh, "🇨🇳 中文");
                        });
                        
                    if ui.button(texts.lock_btn).clicked() {
                        self.vault = None;
                        self.master_password.clear();
                        self.entries.clear();
                    }
                    if ui.button(texts.refresh_btn).clicked() {
                        self.refresh_entries();
                    }
                    if ui.button(texts.check_updates_btn).clicked() {
                        let _ = open::that("https://github.com/shield/shield/releases");
                    }
                    if ui.button(texts.add_new_btn).clicked() {
                        // Reset edit fields
                        self.is_editing = true;
                        self.selected_entry_uuid = None;
                        self.edit_entry_name.clear();
                        self.edit_entry_username.clear();
                        self.edit_entry_password.clear();
                        self.edit_entry_url.clear();
                        self.edit_entry_notes.clear();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Left Panel: Entry List
                ui.vertical(|ui| {
                    ui.set_min_width(250.0);
                    ui.set_max_width(300.0);
                    
                    let filtered_entries: Vec<&Entry> = self.entries.iter()
                        .filter(|e| {
                            self.search_query.is_empty() || 
                            e.name.to_lowercase().contains(&self.search_query.to_lowercase()) ||
                            e.username.as_deref().unwrap_or("").to_lowercase().contains(&self.search_query.to_lowercase())
                        })
                        .collect();

                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for entry in filtered_entries {
                            let selected = Some(entry.uuid) == self.selected_entry_uuid;
                            if ui.selectable_label(selected, &entry.name).clicked() {
                                self.selected_entry_uuid = Some(entry.uuid);
                                self.is_editing = false;
                            }
                        }
                    });
                });

                ui.separator();

                // Right Panel: Details or Edit Form
                if self.is_editing {
                    self.show_edit_form(ui);
                } else if let Some(uuid) = self.selected_entry_uuid {
                    if let Some(entry) = self.entries.iter().find(|e| e.uuid == uuid) {
                        self.show_entry_details(ui, entry.clone());
                    }
                } else {
                    ui.vertical_centered(|ui| {
                        ui.add_space(50.0);
                        ui.label(self.text().select_entry_hint);
                    });
                }
            });
        });
    }

    fn show_entry_details(&mut self, ui: &mut egui::Ui, entry: Entry) {
        let texts = self.text();
        ui.vertical(|ui| {
            ui.heading(&entry.name);
            ui.add_space(10.0);

            egui::Grid::new("details_grid").num_columns(3).spacing([10.0, 10.0]).show(ui, |ui| {
                ui.label(texts.username_label);
                ui.label(entry.username.as_deref().unwrap_or("-"));
                if let Some(username) = &entry.username {
                     if ui.button("📋").on_hover_text(texts.copy_username_tooltip).clicked() {
                         self.copy_to_clipboard(username);
                     }
                }
                ui.end_row();

                ui.label(texts.password_label);
                ui.label("********");
                if ui.button("📋").on_hover_text(texts.copy_password_tooltip).clicked() {
                    self.copy_to_clipboard(entry.password.expose_secret());
                }
                ui.end_row();

                ui.label(texts.url_label);
                ui.label(entry.url.as_deref().unwrap_or("-"));
                ui.end_row();

                ui.label(texts.notes_label);
                ui.label(entry.notes.as_deref().unwrap_or("-"));
                ui.end_row();
            });

            ui.add_space(20.0);
            ui.horizontal(|ui| {
                 if ui.button(texts.edit_btn).clicked() {
                     self.is_editing = true;
                     self.edit_entry_name = entry.name.clone();
                     self.edit_entry_username = entry.username.clone().unwrap_or_default();
                     self.edit_entry_password = entry.password.expose_secret().clone();
                     self.edit_entry_url = entry.url.clone().unwrap_or_default();
                     self.edit_entry_notes = entry.notes.clone().unwrap_or_default();
                 }
                 if ui.button(texts.delete_btn).clicked() {
                     if let Some(vault) = &self.vault {
                         let _ = vault.delete_entry(&entry.uuid);
                         self.selected_entry_uuid = None;
                         self.refresh_entries();
                     }
                 }
            });
        });
    }

    fn show_edit_form(&mut self, ui: &mut egui::Ui) {
        let texts = self.text();
        ui.vertical(|ui| {
            ui.heading(if self.selected_entry_uuid.is_some() { texts.edit_entry_title } else { texts.new_entry_title });
            ui.add_space(10.0);

            egui::Grid::new("edit_grid").num_columns(2).spacing([10.0, 10.0]).show(ui, |ui| {
                ui.label(texts.name_label);
                ui.text_edit_singleline(&mut self.edit_entry_name);
                ui.end_row();

                ui.label(texts.username_label);
                ui.text_edit_singleline(&mut self.edit_entry_username);
                ui.end_row();

                ui.label(texts.password_label);
                ui.text_edit_singleline(&mut self.edit_entry_password);
                ui.end_row();

                ui.label(texts.url_label);
                ui.text_edit_singleline(&mut self.edit_entry_url);
                ui.end_row();

                ui.label(texts.notes_label);
                ui.text_edit_multiline(&mut self.edit_entry_notes);
                ui.end_row();
            });

            ui.add_space(20.0);
            ui.horizontal(|ui| {
                if ui.button(texts.save_btn).clicked() {
                    if let Some(vault) = &self.vault {
                        let username = if self.edit_entry_username.is_empty() { None } else { Some(self.edit_entry_username.clone()) };
                        let url = if self.edit_entry_url.is_empty() { None } else { Some(self.edit_entry_url.clone()) };
                        let notes = if self.edit_entry_notes.is_empty() { None } else { Some(self.edit_entry_notes.clone()) };
                        
                        if let Some(uuid) = self.selected_entry_uuid {
                            // Update existing
                            if let Ok(mut entry) = vault.get_entry(&uuid) {
                                entry.name = self.edit_entry_name.clone();
                                entry.username = username;
                                entry.password = SecretBox::new(Box::new(self.edit_entry_password.clone()));
                                entry.url = url;
                                entry.notes = notes;
                                entry.update_timestamp();
                                let _ = vault.update_entry(&entry);
                            }
                        } else {
                            // Create new
                            let mut entry = Entry::new(self.edit_entry_name.clone(), username, self.edit_entry_password.clone());
                            entry.url = url;
                            entry.notes = notes;
                            let _ = vault.add_entry(&entry);
                            self.selected_entry_uuid = Some(entry.uuid);
                        }
                        self.is_editing = false;
                        self.refresh_entries();
                    }
                }
                if ui.button(texts.cancel_btn).clicked() {
                    self.is_editing = false;
                    if self.selected_entry_uuid.is_none() {
                        // If cancelling creation of new entry, clear selection
                    }
                }
            });
        });
    }
}
