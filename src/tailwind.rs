use leptos_struct_table::{ColumnSort, TableClassesProvider};

#[derive(Clone, Copy)]
pub struct TailwindClassesPreset;

impl TableClassesProvider for TailwindClassesPreset {
    fn new() -> Self {
        Self
    }

    fn table(&self, classes: &str) -> String {
        format!(
            "{} {}",
            "text-sm text-left text-dark-500 dark:text-dark-400", classes
        )
    }

    fn head_row(&self, template_classes: &str) -> String {
        format!(
            "{} {}",
            "sticky top-0 text-xs text-dark-700 uppercase bg-dark-200 dark:bg-dark-700 dark:text-dark-300",
            template_classes
        )
    }

    fn head_cell(&self, sort: ColumnSort, template_classes: &str) -> String {
        let sort_class = match sort {
            ColumnSort::None => "",
            _ => "text-darker dark:text-light",
        };

        format!(
            "cursor-pointer px-5 py-2 {} {}",
            sort_class, template_classes
        )
    }

    fn head_cell_inner(&self) -> String {
        "flex items-center after:content-[--sort-icon] after:pl-1 after:opacity-40 before:content-[--sort-priority] before:order-last before:pl-0.5 before:font-light before:opacity-40".to_string()
    }

    fn row(&self, row_index: usize, selected: bool, template_classes: &str) -> String {
        let bg_color = if row_index % 2 == 0 {
            if selected {
                "bg-cyan-300 text-dark-700 dark:bg-cyan-700 dark:text-dark-400"
            } else {
                "bg-light dark:bg-dark-900 hover:bg-dark-100 dark:hover:bg-dark-800"
            }
        } else {
            if selected {
                "bg-cyan-300 text-dark-700 dark:bg-cyan-700 dark:text-dark-400"
            } else {
                "bg-dark-50 dark:bg-dark-800 hover:bg-dark-100 dark:hover:bg-dark-700"
            }
        };

        format!(
            "{} {} {}",
            "border-b dark:border-dark-700", bg_color, template_classes
        )
    }

    fn cell(&self, template_classes: &str) -> String {
        format!("{} {}", "px-5 py-2", template_classes)
    }
}
