use std::fs::File;
use std::io;

use crate::js::handlers::generic_tokens::*;
use crate::js::node::*;
use crate::js::custom_buf_writer::*;
use crate::js::handlers::OBJECT::OBJECT;

pub struct ROOM {}

impl HandleJS for ROOM {
    fn print (root: &JSNode, mut writer: &mut CustomBufWriter<File>) -> Result<(), io::Error> {      
        writer.w("let ")?;
        W::print(&root.children[0], &mut writer)?;
        writer.w(" = {\n")?;

        for i in 1..root.children.len() {
            match &root.children[i].children[0].value[..] {
                "PSEUDO" => Self::format_obj(&root.children[i], &mut writer)?,
                "DESC" | "LDESC" | "ACTION" | "IN" => OBJECT::format_string(&root.children[i], &mut writer)?,
                "FLAGS" => OBJECT::format_flag_object(&root.children[i], &mut writer)?,
                "GLOBAL" => OBJECT::format_array_of_strings(&root.children[i], &mut writer)?,
                "VALUE" => OBJECT::format_int(&root.children[i], &mut writer)?,
                "NORTH_TO" | "NE_TO" | "EAST_TO" | "SE_TO" | "SOUTH_TO" | "SW_TO" | "WEST_TO" | "NW_TO" | "LAND_TO" | "UP_TO" | "DOWN_TO" | "OUT_TO" | "IN_TO" => Self::format_direction(&root.children[i], &mut writer)?,
                _ => (),
            };
        }

        writer.w("};\n")?;

        Ok(())
    }
}

impl ROOM {
    pub fn format_obj(root: &JSNode, mut writer: &mut CustomBufWriter<File>) -> Result<(), io::Error> {  
      W::print(&root.children[0], &mut writer)?;
      writer.w(": {\n")?;

      for i in 1..root.children.len() {
        if i%2 == 0 {
          W::print_with_quotes(&root.children[i], &mut writer)?;
          writer.w(",\n")?;
        } else {
          T::print(&root.children[i], &mut writer)?;
          writer.w(": ")?;
        }
      }
  
      writer.w("},\n")?;
  
      Ok(())
    }

    pub fn format_direction(root: &JSNode, mut writer: &mut CustomBufWriter<File>) -> Result<(), io::Error> { 
      /*
      (IN_TO <TELL "The dam blocks your way.">)
      (IN_TO SQUEEKY-ROOM)
      (IN_TO <GRATING-EXIT>)
      (IN_TO <COND (,WON-FLAG STONE-BARROW)>)
      (IN_TO <COND (<FSET? ,KITCHEN-WINDOW ,OPENBIT> KITCHEN)>)
      (IN_TO <COND (,LOW-TIDE RESERVOIR) (T <TELL "You would drown">)>)
      (IN_TO <COND (<FSET? ,Y ,Z> X) (T <TELL "Text">)>)
      */

      // return string if can go to another room
      // else return nothing

      W::print(&root.children[0], &mut writer)?;
      writer.w(": () => \n")?;

      match root.children[1].kind {
        JSNodeType::Routine => R::print(&root.children[1], &mut writer)?,
        JSNodeType::Word => W::print_with_quotes(&root.children[1], &mut writer)?,
        _ => (),
      }

      writer.w(",\n")?;
      
      Ok(())
    }
}

