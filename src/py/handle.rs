use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

//use crate::zil::tokenize::*;
use crate::zil::ast::*;

pub fn handle_r(root: &Node, indent: u64, mut writer: &mut BufWriter<File>) -> Result<(), ()> {
    if !root.is_routine() {
        return Err(());
    }

    let spacer = (0..indent).map(|_| "  ").collect::<String>(); 

    if root.children.len() <= 0 {
        writer.write(format!("{}None", spacer).as_bytes());
        return Ok(());
    }

    match &root.children[0].tokens[0].value[..] {
        "COND" => handle_r_COND(&root, indent, &mut writer),
        "TELL" => handle_r_TELL(&root, indent, &mut writer),
        "ROUTINE" => handle_r_ROUTINE(&root, indent, &mut writer),
        v => {
            writer.write(format!("{}{}(", spacer, v).as_bytes());
            for i in 1..root.children.len() {
                match root.children[i].kind() {
                    NodeType::Routine => handle_r(&root.children[i], 0, &mut writer),
                    NodeType::Grouping => handle_g(&root.children[i], 0, &mut writer),
                    NodeType::Text => handle_t(&root.children[i], 0, &mut writer),
                    NodeType::Word => handle_w(&root.children[i], 0, &mut writer),
                    _ => Err(()),
                };
                if i+1 < root.children.len() {
                    writer.write(b", ");
                }
            }
            writer.write(b")");
            Ok(())
        }
    }?;

    Ok(())
}

fn handle_g(root: &Node, indent: u64, mut writer: &mut BufWriter<File>) -> Result<(), ()> {
    if !root.is_grouping() {
        return Err(());
    }

    let spacer = (0..indent).map(|_| "  ").collect::<String>(); 
    writer.write(format!("{}(", spacer).as_bytes());

    for i in 0..root.children.len() {
        match root.children[i].kind() {
            NodeType::Routine => handle_r(&root.children[i], 0, &mut writer),
            NodeType::Grouping => handle_g(&root.children[i], 0, &mut writer),
            NodeType::Text => handle_t(&root.children[i], 0, &mut writer),
            NodeType::Word => handle_w(&root.children[i], 0, &mut writer),
            _ => Err(()),
        }?;
        if i+1 < root.children.len() {
            writer.write(b" ");
        }
    }

    writer.write(b")");

    Ok(())
}

fn handle_t(root: &Node, indent: u64, mut writer: &mut BufWriter<File>) -> Result<(), ()> {
    if !root.is_text() {
        return Err(());
    }

    let spacer = (0..indent).map(|_| "  ").collect::<String>(); 
    writer.write(format!("{}\"{}\"", spacer, root.tokens[0].value).as_bytes());

    Ok(())
}

fn handle_w(root: &Node, indent: u64, mut writer: &mut BufWriter<File>) -> Result<(), ()> {
    if !root.is_word() {
        return Err(());
    }

    let spacer = (0..indent).map(|_| "  ").collect::<String>(); 
    writer.write(format!("{}{}", spacer, root.tokens[0].value).as_bytes());

    Ok(())
}

fn handle_r_COND(root: &Node, indent: u64, mut writer: &mut BufWriter<File>) -> Result<(), ()> {
    if root.children.len() <= 1 ||
       !root.children[1].is_grouping() || 
       root.children[1].children.len() <= 1 {
        return Err(());
    }

    let spacer = (0..indent).map(|_| "  ").collect::<String>();
    writer.write(format!("{}if ", spacer).as_bytes());

    for g in 1..root.children.len() {
        match root.children[g].children[0].kind() {
            NodeType::Routine => handle_r(&root.children[g].children[0], 0, &mut writer),       
            NodeType::Word => handle_w(&root.children[g].children[0], 0, &mut writer),
            _ => Err(()),
        }?;
        writer.write(b":\n");
    
        for i in 1..root.children[g].children.len() {
            match root.children[g].children[i].kind() {
                NodeType::Routine => handle_r(&root.children[g].children[i], indent+1, &mut writer), // handle_r_in_g_in_r_COND
                NodeType::Word => handle_w(&root.children[g].children[i], indent+1, &mut writer), // handle_w_in_g_in_r_COND
                _ => Err(()),
            }?;
            writer.write(b"\n");
        }

        if g+1 < root.children.len() {
            writer.write(format!("{}elif ", spacer).as_bytes());
        }
    }
    
    Ok(())
}

fn handle_r_TELL(root: &Node, indent: u64, mut writer: &mut BufWriter<File>) -> Result<(), ()> {
    if root.children.len() <= 1 {
        return Err(());
    }

    let spacer = (0..indent).map(|_| "  ").collect::<String>();
    writer.write(format!("{}print(", spacer).as_bytes());

    for i in 1..root.children.len() {
        match root.children[i].kind() {
            NodeType::Routine => handle_r(&root.children[i], 0, &mut writer), // handle_r_in_r_TELL
            NodeType::Text => handle_t(&root.children[i], 0, &mut writer), // handle_t_in_r_TELL
            NodeType::Word => handle_w(&root.children[i], 0, &mut writer), // handle_w_in_r_TELL
            _ => Err(()),
        }?;
        if i+1 < root.children.len() {
            writer.write(b" + ");
        }
    }

    writer.write(b")\n");

    Ok(())
}

/*
<ROUTINE OTVAL-FROB ("OPTIONAL" (O ,TROPHY-CASE) "AUX" F (SCORE 0))
	 <SET F <FIRST? .O>>
	 <REPEAT ()
		 <COND (<NOT .F> <RETURN .SCORE>)>
		 <SET SCORE <+ .SCORE <GETP .F ,P?TVALUE>>>
		 <COND (<FIRST? .F> <OTVAL-FROB .F>)>
         <SET F <NEXT? .F>>>>
*/

fn handle_r_ROUTINE(root: &Node, indent: u64, mut writer: &mut BufWriter<File>) -> Result<(), ()> {
    if root.children.len() < 4 || !root.children[1].is_word() || !root.children[2].is_grouping() {
        return Err(());
    }

    let spacer = (0..indent).map(|_| "  ").collect::<String>();
    writer.write(format!("{0}\n{0}def {1}(", spacer, root.children[1].tokens[0].value).as_bytes());

    enum ArgState {
        INITIAL,
        OPTIONAL,
        AUX,
    }

    let mut param_buf: Vec<&Node> = Vec::new();
    let mut optional_param_buf: Vec<&Node> = Vec::new();
    let mut body_buf: Vec<&Node> = Vec::new();
    let mut arg_state = ArgState::INITIAL;
    for i in 0..root.children[2].children.len() {
        let tmp = &root.children[2].children[i];
        match tmp.kind() {
            NodeType::Grouping => {
                if tmp.children.len() != 2 || !tmp.children[0].is_word() {
                    return Err(());
                }
                match arg_state {
                    ArgState::INITIAL => param_buf.push(&tmp),
                    ArgState::OPTIONAL => optional_param_buf.push(&tmp),
                    ArgState::AUX => body_buf.push(&tmp),
                };
            },
            NodeType::Text => {
                match &tmp.tokens[0].value[..] {
                    "AUX" => arg_state = ArgState::AUX,
                    "OPTIONAL" => arg_state = ArgState::OPTIONAL,
                    _ => return Err(()),
                };
            },
            NodeType::Word => {
                match arg_state {
                    ArgState::INITIAL => param_buf.push(&tmp),
                    ArgState::OPTIONAL => optional_param_buf.push(&tmp),
                    ArgState::AUX => body_buf.push(&tmp),
                };
            }
            _ => return Err(()),
        };
    }

    if param_buf.len() > 0 {
        for i in 0..param_buf.len() {
            match param_buf[i].kind() {
                NodeType::Grouping => handle_w(&param_buf[i].children[0], 0, &mut writer),
                NodeType::Word => handle_w(&param_buf[i], 0, &mut writer),
                _ => Err(()),
            }?;
            if i+1 < param_buf.len() {
                writer.write(b", ");
            }
        }
    }

    if optional_param_buf.len() > 0 {
        if param_buf.len() > 0 {
            writer.write(b", ");
        }
        for i in 0..optional_param_buf.len() {
            match optional_param_buf[i].kind() {
                NodeType::Grouping => {
                    handle_w(&optional_param_buf[i].children[0], 0, &mut writer)?;
                    writer.write(b"=");
                    match optional_param_buf[i].children[1].kind() {
                        NodeType::Routine => handle_r(&optional_param_buf[i].children[1], 0, &mut writer),
                        NodeType::Text => handle_t(&optional_param_buf[i].children[1], 0, &mut writer),
                        NodeType::Word => handle_w(&optional_param_buf[i].children[1], 0, &mut writer),
                        _ => Err(()),
                    }?;
                },
                NodeType::Word => {
                    handle_w(&optional_param_buf[i], 0, &mut writer)?;
                    writer.write(b"=None");
                },
                _ => return Err(()),
            };
            if i+1 < param_buf.len() {
                writer.write(b", ");
            }
        }
    }

    writer.write(b"):\n");

    if body_buf.len() > 0 {
        for i in 0..body_buf.len() {
            writer.write(format!("{}  ", spacer).as_bytes());
            match body_buf[i].kind() {
                NodeType::Grouping => {
                    handle_w(&body_buf[i].children[0], 0, &mut writer)?;
                    writer.write(b" = ");
                    match body_buf[i].children[1].kind() {
                        NodeType::Routine => handle_r(&body_buf[i].children[1], 0, &mut writer),
                        NodeType::Text => handle_t(&body_buf[i].children[1], 0, &mut writer),
                        NodeType::Word => handle_w(&body_buf[i].children[1], 0, &mut writer),
                        _ => Err(()),
                    }?;
                },
                NodeType::Word => {
                    handle_w(&body_buf[i], 0, &mut writer)?;
                    writer.write(b" = None");
                },
                _ => return Err(()),
            };
            writer.write(b"\n");
        }
    }

    for i in 3..root.children.len() {
        match root.children[i].kind() {
            NodeType::Routine => handle_r(&root.children[i], indent+1, &mut writer),
            _ => Err(()),
        }?;
    }

    Ok(())
}