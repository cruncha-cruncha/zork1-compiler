use std::fs::File;

use crate::zil::contracts::*;
use crate::js::contracts::*;
use crate::js::helpers::*;
use crate::js::custom_buf_writer::*;

// <CONSTANT ... >

// focus on
// 1actions.zil
// 1dungeon.zil
// gglobals.zil
// gsyntax.zil
// gverbs.zil

pub struct R {} // routine, aka any <>
pub struct G {} // grouping, aka any ()
pub struct T {} // text
pub struct W {} // word

impl HandleJS for R {
    fn validate (root: &ZilNode) -> Result<(), HandlerErr> {
        if !root.is_routine() {
            return Err(HandlerErr::origin(format!("Invalid generic routine: {}", root)));
        }
        Ok(())
    }
  
    fn print (root: &ZilNode, indent: u64, mut writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
        Self::validate(root)?;

        let spacer = (0..indent).map(|_| "  ").collect::<String>(); 
    
        if root.children.len() <= 0 {
            wrap!(writer.w(format!("{}null", spacer)));
            return Ok(());
        }
    
        match &root.children[0].tokens[0].value[..] {
            "INSERT-FILE" => crate::js::handlers::import::Import::print(&root, indent, &mut writer),
            "GLOBAL" => crate::js::handlers::GLOBAL::GLOBAL::print(&root, indent, &mut writer),
            "TABLE" | "LTABLE" => crate::js::handlers::table::Table::print(&root, indent, &mut writer),
            "COND" => crate::js::handlers::COND::COND::print(&root, indent, &mut writer),
            "ROUTINE" => crate::js::handlers::ROUTINE::ROUTINE::print(&root, indent, &mut writer),
            "REPEAT" => crate::js::handlers::REPEAT::REPEAT::print(&root, indent, &mut writer),
            "OBJECT" => crate::js::handlers::OBJECT::OBJECT::print(&root, indent, &mut writer),
            "ROOM" => crate::js::handlers::ROOM::ROOM::print(&root, indent, &mut writer),
            "TELL" => crate::js::handlers::TELL::TELL::print(&root, indent, &mut writer),
            "SET" => crate::js::handlers::SET::SET::print(&root, indent, &mut writer),
            "EQUAL?" | "==?" | "=?" => crate::js::handlers::equals::Equals::print(&root, indent, &mut writer),
            "AND" => crate::js::handlers::AND::AND::print(&root, indent, &mut writer),
            "OR" => crate::js::handlers::OR::OR::print(&root, indent, &mut writer),
            "NOT" => crate::js::handlers::NOT::NOT::print(&root, indent, &mut writer),
            "+" => crate::js::handlers::add::Add::print(&root, indent, &mut writer),
            "-" => crate::js::handlers::subtract::Subtract::print(&root, indent, &mut writer),
            "*" => crate::js::handlers::multiply::Multiply::print(&root, indent, &mut writer),
            "/" => crate::js::handlers::divide::Divide::print(&root, indent, &mut writer),
            _ => {
                wrap!(writer.w(format!("{}", spacer)));
                wrap!(W::print(&root.children[0], 0, &mut writer));
                wrap!(writer.w("("));
                for i in 1..root.children.len() {
                    match root.children[i].kind() {
                        ZilNodeType::Routine => wrap!(R::print(&root.children[i], 0, &mut writer)),
                        ZilNodeType::Grouping => wrap!(G::print(&root.children[i], 0, &mut writer)),
                        ZilNodeType::Text => wrap!(T::print(&root.children[i], 0, &mut writer)),
                        ZilNodeType::Word => wrap!(W::print(&root.children[i], 0, &mut writer)),
                        _ => return Err(OutputErr::from(HandlerErr::origin("could not handle generic routine"))),
                    };
                    if i+1 < root.children.len() {
                        wrap!(writer.w(", "));
                    }
                }
                wrap!(writer.w(")"));
                Ok(())
            }
        }?;
    
        Ok(())
    }
}

impl HandleJS for G {
    fn validate (root: &ZilNode) -> Result<(), HandlerErr> {
        if !root.is_grouping() {
            return Err(HandlerErr::origin(format!("Invalid generic grouping: {}", root)));
        }
        Ok(())
    }
  
    fn print (root: &ZilNode, indent: u64, mut writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
        Self::validate(root)?;

        let spacer = (0..indent).map(|_| "  ").collect::<String>(); 
        wrap!(writer.w(format!("{}(", spacer)));
    
        for i in 0..root.children.len() {
            match root.children[i].kind() {
                ZilNodeType::Routine => wrap!(R::print(&root.children[i], 0, &mut writer)),
                ZilNodeType::Grouping => wrap!(G::print(&root.children[i], 0, &mut writer)),
                ZilNodeType::Text => wrap!(T::print(&root.children[i], 0, &mut writer)),
                ZilNodeType::Word => wrap!(W::print(&root.children[i], 0, &mut writer)),
                _ => return Err(OutputErr::from(HandlerErr::origin("Cannot print unknown ZilNodeType in generic grouping"))),
            };
            if i+1 < root.children.len() {
                wrap!(writer.w(" "));
            }
        }
    
        wrap!(writer.w(")"));
    
        Ok(())
    }
}

impl HandleJS for T {
    fn validate (root: &ZilNode) -> Result<(), HandlerErr> {
        if !root.is_text() {
            return Err(HandlerErr::origin(format!("Invalid generic text: {}", root)));
        }
        Ok(())
    }
  
    fn print (root: &ZilNode, indent: u64, writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
        Self::validate(root)?;
        
        let spacer = (0..indent).map(|_| "  ").collect::<String>(); 
        let text = wrap!(escape_text(&root), root);
        wrap!(writer.w(format!("{}\"{}\"", spacer, text)), root);
    
        Ok(())
    }
}

impl HandleJS for W {
    fn validate (root: &ZilNode) -> Result<(), HandlerErr> {
        if !root.is_word() {
            return Err(HandlerErr::origin(format!("Invalid generic word: {}", root)));
        }
        Ok(())
    }
  
    fn print (root: &ZilNode, indent: u64, writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
        Self::validate(root)?;

        let spacer = (0..indent).map(|_| "  ").collect::<String>(); 
        let keyword = wrap!(format_keyword(&root), root);
        wrap!(writer.w(format!("{}{}", spacer, keyword)), root);
    
        Ok(())
    }
}

impl T {
    // not recommended
    pub fn print_as_word (root: &ZilNode, indent: u64, writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
        Self::validate(root)?;
        
        let spacer = (0..indent).map(|_| "  ").collect::<String>(); 
        let text = wrap!(format_keyword(&root), root);
        wrap!(writer.w(format!("{}{}", spacer, text)), root);
    
        Ok(())
    }
}

impl W {
    // not recommended
    pub fn print_with_quotes (root: &ZilNode, indent: u64, writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
        Self::validate(root)?;

        let spacer = (0..indent).map(|_| "  ").collect::<String>(); 
        let keyword = wrap!(format_keyword(&root), root);
        wrap!(writer.w(format!("{}\"{}\"", spacer, keyword)), root);
    
        Ok(())
    }
}