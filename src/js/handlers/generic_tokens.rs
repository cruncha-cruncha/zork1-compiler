use std::fs::File;
use std::io;

use crate::js::node::*;
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
    fn print (root: &JSNode, mut writer: &mut CustomBufWriter<File>) -> Result<(), io::Error> {    
        if root.kind == JSNodeType::EmptyRoutine {
            writer.w(format!("null"))?;
            return Ok(());
        }
    
        match &root.value[..] {
            "ROUTINE" => crate::js::handlers::ROUTINE::ROUTINE::print(&root, &mut writer),
            "REPEAT" => crate::js::handlers::REPEAT::REPEAT::print(&root, &mut writer),
            "COND" => crate::js::handlers::COND::COND::print(&root, &mut writer),
            "OBJECT" => crate::js::handlers::OBJECT::OBJECT::print(&root, &mut writer),
            "ROOM" => crate::js::handlers::ROOM::ROOM::print(&root, &mut writer),
            _ => {
                writer.w(format!("{}", format_keyword(&root.value).unwrap()))?;
                writer.w("(")?;
                for i in 0..root.children.len() {
                    match root.children[i].kind {
                        JSNodeType::Routine | JSNodeType::EmptyRoutine => R::print(&root.children[i], &mut writer),
                        JSNodeType::Grouping => G::print(&root.children[i], &mut writer),
                        JSNodeType::Text => T::print(&root.children[i], &mut writer),
                        JSNodeType::Word | JSNodeType::Int => W::print(&root.children[i], &mut writer),
                        _ => panic!(),
                    }?;
                    if i+1 < root.children.len() {
                        writer.w(", ")?;
                    }
                }
                writer.w(")\n")?;
                Ok(())
            }
        }?;
    
        Ok(())
    }
}

impl HandleJS for G {
    fn print (root: &JSNode, mut writer: &mut CustomBufWriter<File>) -> Result<(), io::Error> {
        writer.w("(")?;
    
        for i in 0..root.children.len() {
            match root.children[i].kind {
                JSNodeType::Routine | JSNodeType::EmptyRoutine => R::print(&root.children[i], &mut writer)?,
                JSNodeType::Grouping => G::print(&root.children[i], &mut writer)?,
                JSNodeType::Text => T::print(&root.children[i], &mut writer)?,
                JSNodeType::Word | JSNodeType::Int => W::print(&root.children[i], &mut writer)?,
                _ => panic!(),
            };
            if i+1 < root.children.len() {
                writer.w(" ")?;
            }
        }
    
        writer.w(")")?;
    
        Ok(())
    }
}

impl HandleJS for T {
    fn print (root: &JSNode, writer: &mut CustomBufWriter<File>) -> Result<(), io::Error> {        
        let text = escape_text(&root.value);
        writer.w(format!("\"{}\"", text))?;
        Ok(())
    }
}

impl HandleJS for W {
    fn print (root: &JSNode, writer: &mut CustomBufWriter<File>) -> Result<(), io::Error> {
        let keyword = format_keyword(&root.value).unwrap();
        writer.w(format!("{}", keyword))?;
        Ok(())
    }
}

impl T {
    // not recommended
    #[allow(dead_code)]
    pub fn print_as_word (root: &JSNode, writer: &mut CustomBufWriter<File>) -> Result<(), io::Error> {
        let keyword = format_keyword(&root.value).unwrap();
        writer.w(format!("{}", keyword))?;
        Ok(())
    }
}

impl W {
    // not recommended
    #[allow(dead_code)]
    pub fn print_as_text (root: &JSNode, writer: &mut CustomBufWriter<File>) -> Result<(), io::Error> {
        let text = escape_text(&root.value);
        writer.w(format!("\"{}\"", text))?;
        Ok(())
    }

    // not recommended
    #[allow(dead_code)]
    pub fn print_with_quotes (root: &JSNode, writer: &mut CustomBufWriter<File>) -> Result<(), io::Error> {
        let keyword = format_keyword(&root.value).unwrap();
        writer.w(format!("\"{}\"", keyword))?;
        Ok(())
    }
}