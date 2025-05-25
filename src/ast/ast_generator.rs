use std::fs::File;
use std::io::{Result, Write};

pub struct AstGenerator;

impl AstGenerator {
    pub fn define_ast(output_dir: &str, base_name: &str, types: &[&str]) -> Result<()> {
        let path = format!("{}/{}.rs", output_dir, base_name.to_lowercase());
        let mut writer = File::create(&path)?;

        writeln!(writer, "// This is auto-generated code.")?;
        writeln!(writer, "use crate::token::Token;")?;
        writeln!(writer, "use crate::ast::Literal;\n")?;

        // AST enum
        writeln!(writer, "#[derive(Debug, Clone)]")?;
        writeln!(writer, "pub enum {} {{", base_name)?;
        for &ty in types {
            let class_name = ty.split(':').next().unwrap().trim();
            let fields = ty.split(':').nth(1).unwrap().trim();
            Self::define_type(&mut writer, class_name, fields, base_name)?;
        }
        writeln!(writer, "}}\n")?;

        // Visitor trait
        Self::define_visitor(&mut writer, base_name, types)?;

        // Accept method
        Self::define_accept(&mut writer, base_name, types)?;

        Ok(())
    }

    fn define_type(
        writer: &mut File,
        class_name: &str,
        field_list: &str,
        base_name: &str,
    ) -> Result<()> {
        let fields: Vec<&str> = field_list.split(", ").collect();
        let mut field_defs = Vec::new();

        for field in fields {
            let parts: Vec<&str> = field.split_whitespace().collect();
            if parts.len() != 2 {
                panic!("Invalid field format: '{}'", field);
            }

            let ty = parts[0];
            let name = parts[1];
            let rust_type = if ty == base_name {
                format!("Box<{}>", ty)
            } else {
                ty.to_string()
            };
            field_defs.push(format!("{}: {}", name, rust_type));
        }

        writeln!(writer, "    {}({}),", class_name, field_defs.join(", "))?;
        Ok(())
    }

    fn define_visitor(writer: &mut File, base_name: &str, types: &[&str]) -> Result<()> {
        writeln!(writer, "pub trait Visitor<R> {{")?;
        for &ty in types {
            let class_name = ty.split(':').next().unwrap().trim();
            writeln!(
                writer,
                "    fn visit_{}_{}(&mut self, node: &{}) -> R;",
                class_name.to_lowercase(),
                base_name.to_lowercase(),
                base_name
            )?;
        }
        writeln!(writer, "}}\n")?;
        Ok(())
    }

    fn define_accept(writer: &mut File, base_name: &str, types: &[&str]) -> Result<()> {
        writeln!(writer, "impl {} {{", base_name)?;
        writeln!(
            writer,
            "    pub fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {{"
        )?;
        writeln!(writer, "        match self {{")?;
        for &ty in types {
            let class_name = ty.split(':').next().unwrap().trim();
            writeln!(
                writer,
                "            {}::{}(..) => visitor.visit_{}_{}(self),",
                base_name,
                class_name,
                class_name.to_lowercase(),
                base_name.to_lowercase()
            )?;
        }
        writeln!(writer, "        }}")?;
        writeln!(writer, "    }}")?;
        writeln!(writer, "}}")?;
        Ok(())
    }
}
