use std::collections::HashMap;
use std::fs::{self, DirEntry, File};
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {

    println!("Instructions:");
    println!("- By now, you should have the folder with the new generated translations, and, your completed translation folder somewhere else or with other name.");
    println!("- First, enter the folder or file with your translation, then the new one with the new keys.");
    println!("- The output will be one file with the applied changes and another with the detected changes.");
    println!("- Folder mode will recreate the folder structure, so you can move its contents by overwriting the contents of the new folder.");
    println!("Notes:");
    println!("- A deleted or added line is indistinguishable from a dialogue slightly changed.");
    println!("- The files are searched for by their path. If the developer moved a file to a different folder, or simply renamed it, they will be treated as new files. You can process those files individually later.");
    println!("- The processed folder will be in the same location as the new folder, adding \"_replaced\" to the end of the name.");
    println!("- Folders with the name \"_replaced\" will be ignored.");

    let mode = get_bool_for_input_1_or_2("Select mode:\n(1) - file\n(2) - folder","file","folder");

    if mode{
        return file_mode();
    }
    else {
        return folder_mode();
    }
}

fn wait_input() { println!("\nEnter to continue");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
fn get_input() ->io::Result<String>{
    let mut input = String::new();
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    // input.trim();
    Ok(input)
}
fn get_bool_for_input_1_or_2(indication:&str,true_print:&str, false_print:&str) ->bool{
    loop {
        println!("{}",indication);
        let mut _bool = get_input().unwrap();
        // _bool = _bool.trim();
        
        match _bool.trim() {
            "1" => {println!("{}",true_print) ;return true},
            "2" => {println!("{}",false_print) ;return false},
            _ => eprintln!("Invalid input")
        }
    }
}
fn get_path_input(message:&str, is_file: bool) -> String {
    loop{
        println!("{}",message);
        let _path = get_input().unwrap();
        let path=_path.trim_end_matches(|c| c == '\r' || c == '\n' || c == '/' || c == '\\').replace('\\', "/");
        // gracias a la doc que tenia un ejemplo de closure, sino iban a ser varios trim_end_matches()
        println!("path: {:?}",Path::new(&path));
        if is_file{
            if Path::new(&path).is_file(){break path}
            else {println!("invalid input")}    }
        else{
            if Path::new(&path).is_dir(){break path}
            else {println!("invalid input")}    }
            // if Path::new(&path).exists(){break path}
            // Habia tenido que poner eso porque los is_... no funcionaban, saber porque, pero en 23-10-2025 si
    }
}
fn set_dir_entries(path:&Path, base :&Path,map:&mut HashMap<String, String>) -> io::Result<()>{
    // println!("readelion episode 1: angle's attack");
    for entry in fs::read_dir(path)? {
        let entry: DirEntry = entry?;
        let _entry: PathBuf = entry.path();
        let entry_string = _entry.to_string_lossy().to_string();
        
        if _entry.is_file() && _entry.extension().unwrap_or_default() == "rpy" {
            let clave = _entry.strip_prefix(base).unwrap_or(&_entry).to_string_lossy().to_string();
            map.insert(clave, entry_string);
        }
        else if path.is_dir(){
            if entry_string.ends_with("_replaced"){continue}
            let _= set_dir_entries(&_entry, base, map);
        }
        else {
            println!("none:{}", entry_string);
        }
        }
        // println!("end of readelion");
    Ok(())}

fn folder_mode() -> io::Result<()>{
    println!("Do you want the diffs in one file, or diffs files for each file?");
    let diff_in_one_file = get_bool_for_input_1_or_2("Select mode:\n(1) - one diffs file \n(2) - many diffs files", "one file", "many files");

    let binding = get_path_input("Enter old folder:", false);
    let old_path = Path::new(&binding);
    let mut old_files: HashMap<String, String> = HashMap::new();
    let _= set_dir_entries(&old_path,&old_path,&mut old_files);
    println!("old file list: ");
    for key in old_files.keys() {println!("{}", key);}


    let binding = get_path_input("Enter new folder:", false);
    let new_path = Path::new(&binding);
    let mut new_files: HashMap<String, String> = HashMap::new();
    let _= set_dir_entries(&new_path,&new_path,&mut new_files);
    println!("new file list: ");
    for key in new_files.keys() {println!("{}", key);}

    //TODO: files to exclude
    //y que introduzcas las keys de archivos que no quieras que se procesen 
    wait_input();

    let mut differences_in_file: Vec<String>=Vec::new();
    let mut no_matching_files: Vec<String>= Vec::new();
    let mut no_changed_file: Vec<String>= Vec::new();
    no_matching_files.push("Files without old version:".to_string());
    no_changed_file.push("Files without changes:".to_string());

    let output_path = new_path.parent().unwrap().join("_replaced");
    fs::create_dir_all(&output_path).unwrap();

    for new_file in &new_files{
        // let old_file = old_files.get(&new_file.0);
        if let Some(old_file) = old_files.get(new_file.0){
            println!("found file match: {}",new_file.0);//deberia comparar todos primero,luego meterlo en tupla y dejar los no matching.
            let (processed_content, diff) = process_file(&old_file,new_file.1);//, diff_in_one_file).unwrap()
            
            let _new =Path::new(&new_file.1);
            let name= _new.file_name().unwrap().to_str().unwrap();
            let relative = _new.parent().unwrap().strip_prefix(new_path).unwrap();
            let new_output = output_path.join(relative);
            fs::create_dir_all(&new_output).unwrap();

            if diff.is_empty() {
                println!("No changes in file: {}", new_file.1);
                no_changed_file.push(format!("{}",new_file.1));
                let content = fs::read_to_string(old_file)?;
                let _ = fs::write(new_output, content);
            }
            else{
                create_processed_copy(new_output.to_str().unwrap(), name, &processed_content);
                if diff_in_one_file{
                    differences_in_file.push(format!("In file: {{{}}} > {{{}}}", old_file, new_file.1));
                    differences_in_file.extend(diff);
                    differences_in_file.push("".to_string());
                    }
                else {
                    let _ = save_diff(new_output.to_str().unwrap(), name, &diff);
                }
            }
        }
        else {
             no_matching_files.push(new_file.1.to_string());
             println!("no matching with: {}", new_file.1);
            }
        };
        differences_in_file.push(String::default());
        no_matching_files.push(String::default());
        no_matching_files.push("Files without new version:".to_string());
        for old_file in old_files {
            if let Some(_file) = new_files.get(&old_file.0) {continue;}
            else{no_matching_files.push(old_file.1);}
        }
        if diff_in_one_file {
            differences_in_file.push(String::default());
            differences_in_file.extend(no_matching_files);
            differences_in_file.push(String::default());
            differences_in_file.extend(no_changed_file);
            let _ = save_diff(output_path.to_str().unwrap(), "diff.txt", &differences_in_file);
        }
        else {
            no_matching_files.extend(no_changed_file);
            let _ = save_diff(output_path.to_str().unwrap(), "no_matching_files.txt", &no_matching_files);
        }
    wait_input();
    Ok(())
}

fn file_mode() -> io::Result<()>{
    let old_path:String = get_path_input("Enter old file:", true);
    let new_path:String = get_path_input("Enter new file:", true);
    wait_input();
    let (processed_content, diff) = process_file(&old_path, &new_path);//,true

    if diff.is_empty() {
        println!("No changes found in: {new_path}")
    } else {
        let _new_path = Path::new(&new_path);
        if let Some(parent) = _new_path.parent() {
            let outut_path = parent.join("_replaced");
            fs::create_dir_all(&outut_path).unwrap();
            let name = _new_path.file_name().unwrap().to_str().unwrap();
            create_processed_copy(outut_path.to_str().unwrap(), name, &processed_content);
        let _ = save_diff(&(outut_path.to_str().unwrap()), name,&diff);
        }
    }
    wait_input();
    Ok(())
}

fn process_file(old_path: &str, new_path: &str) -> (String, Vec<String>) {
    let old_blocks = parse_translation_file(old_path);
    let new_blocks = parse_translation_file(new_path);
    let mut changes = Vec::new();
    let mut processed_content = String::new();
    // Esto es parte de que se deberia cambiar despues, que envez de un path y abrir,
    // el parse reciba el archivo ya abierto.
    let new_content = fs::read_to_string(new_path).expect("Failed to read new file");
    for line in new_content.lines(){
        if line.contains("# TODO:") {
        processed_content.push_str(line);
        processed_content.push('\n');
        processed_content.push('\n');
        break;
        }
    }
    for new_block in &new_blocks {
        // Añadir comentarios "# game/" que preceden al bloque
        processed_content.push_str(&new_block.comment);
        processed_content.push('\n');
        
        processed_content.push_str(&new_block.header);
        processed_content.push('\n');
        
        // Buscar bloque viejo por HEADER EXACTO
        if let Some(old_block) = old_blocks.iter().find(|b| b.header == new_block.header) {
            if new_block.is_strings_block() {
                let (string_content, string_changes) = process_strings_content(&old_block.lines, &new_block.lines);
                processed_content.push_str(&string_content);
                changes.extend(string_changes);
            } else {
                for line in &old_block.lines {
                    processed_content.push_str(line);
                    processed_content.push('\n');
                }
            }
        } else {
            for line in &new_block.lines {
                processed_content.push_str(line);
                processed_content.push('\n');
            }
            changes.push(format!("{{NEW}} {{{}}} > {{{}}}", new_block.comment, new_block.header));
        }
    }
    
    for old_block in &old_blocks {
        if !new_blocks.iter().any(|b| b.header == old_block.header) {
            changes.push(format!("{{REMOVED}} {{{}}} > {{{}}}", old_block.comment, old_block.header));
        }
    }
    
   (processed_content, changes)
}

fn process_strings_content(old_lines: &[String], new_lines: &[String]) -> (String, Vec<String>) {
    let mut content = String::new();
    let mut changes = Vec::new();
    
    let old_strings = extract_string_pairs(old_lines);
    let new_strings = extract_string_pairs(new_lines);
    
    for new_str in &new_strings {
        // INCLUIR el comentario "# game/" ANTES de cada par old/new
        if !new_str.comment.is_empty() {
            content.push_str(&new_str.comment);
            content.push('\n');
        }
        
        if let Some(old_translation) = old_strings.iter().find(|s| s.old_lines == new_str.old_lines) {
            // String existente - usar traducción vieja
            for line in &old_translation.old_lines {
                content.push_str(line);
                content.push('\n');
            }
            for line in &old_translation.new_lines {
                content.push_str(line);
                content.push('\n');
            }
        } else {
            // String nuevo - usar contenido nuevo
            for line in &new_str.old_lines {
                content.push_str(line);
                content.push('\n');
            }
            for line in &new_str.new_lines {
                content.push_str(line);
                content.push('\n');
            }
            changes.push(format!("{{NEW_STRING}} {{{}}} > {{{}}}", new_str.comment,new_str.old_lines[0]));
        }
    }
    
    for old_str in &old_strings {

        if !new_strings.iter().any(|s| s.old_lines == old_str.old_lines) {
            changes.push(format!("{{REMOVED_STRING}} {{{}}} > {{{}}}", old_str.comment,old_str.old_lines[0]));
        }
    }
    
    (content, changes)
}

fn parse_translation_file(path: &str) -> Vec<TranslationBlock> {
    let content = fs::read_to_string(path).expect("Failed to read file");
    let mut blocks = Vec::new();
    let mut current_block = TranslationBlock::new();
    let mut comment = String::new();
    
    for line in content.lines() {
        let trimmed = line.trim();
        
        if trimmed.starts_with("translate") {
            if !current_block.header.is_empty() {
                blocks.push(current_block);
                current_block = TranslationBlock::new();
            }
            current_block.header = line.to_string();
            current_block.comment = comment;
            comment = String::new();
        } else if trimmed.starts_with("# game/") {
            comment = String::from(line);
            if current_block.is_strings_block() {
                // DENTRO de bloque strings → el comentario va a lines
                current_block.lines.push(line.to_string());
            } else {
                // FUERA de bloque strings → va a comment
                comment = line.to_string(); // Asignar String
            }
        } else if !current_block.header.is_empty() {
            current_block.lines.push(line.to_string());
        }
    }
    
    if !current_block.header.is_empty() {
        blocks.push(current_block);
    }
    
    blocks
}

// Mejorar la extracción de pares strings
fn extract_string_pairs(lines: &[String]) -> Vec<StringPair> {
    let mut pairs = Vec::new();
    let mut current_old = Vec::new();
    let mut current_new = Vec::new();
    let mut current_comment = String::new();
    let mut in_old = false;
    let mut in_new = false;
    
    for line in lines {
        let trimmed = line.trim();
        
        if trimmed.starts_with("# game/") {
            // COMENTARIO: reiniciar y guardar para el próximo par
            if !current_old.is_empty() || !current_new.is_empty() {
                pairs.push(StringPair {
                    comment: current_comment.clone(), // Usar el comentario actual
                    old_lines: current_old,
                    new_lines: current_new,
                });
                current_old = Vec::new();
                current_new = Vec::new();
            }
            current_comment = line.to_string(); // Guardar NUEVO comentario
            in_old = false;
            in_new = false;
        } else if trimmed.starts_with("old \"") {
            if !current_old.is_empty() || !current_new.is_empty() {
                pairs.push(StringPair {
                    comment: current_comment.clone(),
                    old_lines: current_old,
                    new_lines: current_new,
                });
                current_old = Vec::new();
                current_new = Vec::new();
                current_comment.clear(); // Limpiar comentario después de usarlo
            }
            current_old.push(line.to_string());
            in_old = true;
            in_new = false;
        } else if trimmed.starts_with("new \"") || trimmed.starts_with("new _p(") {
            current_new.push(line.to_string());
            in_new = true;
            in_old = false;
        } else if in_old {
            current_old.push(line.to_string());
        } else if in_new {
            current_new.push(line.to_string());
        }
    }
    
    if !current_old.is_empty() || !current_new.is_empty() {
        pairs.push(StringPair {
            comment: current_comment,
            old_lines: current_old,
            new_lines: current_new,
        });
    }
    
    pairs
}

fn save_diff(new_path: &str, name : &str, diff: &Vec<String>) -> io::Result<()> {
    let output_path = PathBuf::from(new_path).join(name.replace(".rpy", ".txt"));

    let output_file = File::create(&output_path)?;
    let mut writer = BufWriter::new(output_file);
    
    for discrepancia in diff {
        writeln!(writer, "{}", discrepancia)?;
    }
    writer.flush()?;
    println!("Saved diff at: {}", output_path.display());
    Ok(())
}
fn create_processed_copy(new_path: &str, name: &str, processed_content: &str){
    let path = Path::new(new_path);
    let processed_path = path.join(name);//parent_dir.join(name);
    if let Err(e) = fs::write(&processed_path, processed_content){
        eprintln!("Failed to create processed file: {}", e);
    }
    else {println!("Saved file at: {:?}",processed_path.to_str().unwrap())}
}

#[derive(Debug, PartialEq)]
struct TranslationBlock {
    header: String,
    lines: Vec<String>,
    comment: String, // Solo líneas "# game/" que preceden al bloque
}

impl TranslationBlock {
    fn new() -> Self {
        Self {
            header: String::new(),
            lines: Vec::new(),
            comment: String::new(),
        }
    }
    
    fn is_strings_block(&self) -> bool {
        self.header.contains("strings:")
    }
}

#[derive(Debug, PartialEq)]
struct StringPair {
    comment: String,
    old_lines: Vec<String>, // Líneas que componen el old (pueden ser múltiples)
    new_lines: Vec<String>, // Líneas que componen el new (pueden ser múltiples)
}

    //tal vez sea de que primero compare la clave en su misma posicion, y si no coinciden mire todas
    //si la encuentra, la pone y cambia la clave original dispar a otra lista,
    //esa tambien se comparara si no hay coincidencias
    //y si nunca encuentra posicion en la original sale en una lista de claves eliminadas
    //no espera
    //¿eso pasaria tambien con los cambios normales?
    //no no, eso es solo si no estaba esa en el orden, pero habia otra que si,
    //no pasaria con un cambio normal porque la clave nueva no matchearia con una vieja
    //
    //deveria tambien ver si se puede hacer mas automatizado todavia,
    //que solo metes la carpeta del idioma y procesa todos, haciendo una llamada `idioma_new` al lado,
    //con todo el contenido ordenado, respetando nombres de carpetas y todo
    //`to process 1 file press 1`
    //`to process a folder press 2`
    //
    //recordar que donde inician los old new, primero pone como clave de traduccion "strings"
    //quizas podria usar lo de comprobar la clave (el [2] del .slit) para que cambie logica
    //
    //ya mas loco despues, podria hacer que en vez de tener las 2 carpetas y te genere una tercera
    //metes el link de un repositorio o lo que verga sea, y asi solo tienes 1 y genera la segunda
    // 
    //talvez debe hacer que las old_new esten en otra lista, y se procesen todo con otra funcion
    //osea, que se ignoren al comparar y demas las otras.

    