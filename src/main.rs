use std::fs::{self, DirEntry, File};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
// use std::ops::Index;
use std::path::{PathBuf, Path};
// use std::process::Output;
// use std::env;

fn main() -> io::Result<()> {

    println!("Instructions:");
    println!("- By now, you should have your translation folder somewhere else. \nIf you want to keep it in the \"tl\" folder, along with the new generation, rename it with an underscore or something similar.\nNext, you should have the folder with the new generation of translations.");
    println!("- First, enter the folder with your translation, then the new empty folder with the new keys.");
    println!("Notes:");
    println!("- It's not necessary for both folders to be in \"./tl\"; the processed folder will be in the same location as the new folder, adding \"_new\" to the end of the name.");
    println!("- A deleted line is indistinguishable from a changed line, so it can cause subsequent lines to no longer match. You can check the differences to see where the problem starts, then look for that line in the old file and delete it if it's not present in the new one.");

    let mode = get_bool_for_input_1_or_2("Select mode:\n(1) - file\n(2) - folder","file","folder");


    println!("Please be sure that the matching files are in the same rute in both folder.\nIf the dev changed a file path, will no problem, just fix the \"# game/.../file.rpy:\" later");
    println!("Please be sure that the files are correct");

    if mode{
        return file_mode();
    }
    else {
        return folder_mode();
    }
    // Ok(())
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
        let path=_path.trim_end_matches("\r\n").replace('\\', "/");
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
fn set_dir_entries(path:&Path,vec:&mut Vec<DirEntry>) -> io::Result<()>{
    // println!("readelion episode 1: angle's attack");
    for entry in fs::read_dir(path)? {
        let entry: DirEntry = entry?;
        let path: PathBuf = entry.path();
        
        if path.is_file() && path.extension().unwrap_or_default() == "rpy" {
            // println!("file found");
            vec.push(entry);
        }
        else if path.is_dir(){
            // println!("dir found");
            let _= set_dir_entries(&entry.path(), vec);
        }
        else {println!("is not a file or dir")}
    }
    // println!("end of readelion");
    Ok(())
}

fn folder_mode() -> io::Result<()>{
    println!("Do you want the diffs in one file, or diffs files for each file?");
    // println!("Select mode:\n(1) - one diffs file \n(2) - many diffs files\nSelected mode:");
    let diff_in_one_file = get_bool_for_input_1_or_2("Select mode:\n(1) - one diffs file \n(2) - many diffs files", "one file", "many files");

    let old_path:String = get_path_input("Enter old folder:", false);
    let new_path:String = get_path_input("Enter new folder:", false);
    let _ = process_file(&old_path, &new_path, diff_in_one_file);
    
    // let dir = fs::read_dir(old_path)?;
    let mut old_files: Vec<DirEntry> = Vec::new();
    let _= set_dir_entries(&Path::new(&old_path),&mut old_files);
    println!("old file list: {:?}",old_files);

    let mut new_files: Vec<DirEntry> = Vec::new();
    let _= set_dir_entries(&Path::new(&new_path),&mut new_files);
    println!("new file list: {:?}",new_files);
    // for file in new_files{
        
    // }
    todo!();
    // Ok(())
}

fn file_mode() -> io::Result<()>{
    let old_path:String = get_path_input("Enter old file:", true);
    let new_path:String = get_path_input("Enter new file:", true);
    let _ = process_file(&old_path, &new_path,true);
    todo!();
    // Ok(())
}

fn process_file(old_path: &str, new_path: &str, diff_in_one_file:bool) -> io::Result<()>{
    let old_file = File::open(old_path)?;
    let new_file = File::open(new_path)?;
    println!("processing files:\n{}\n>\n{}",old_path,new_path);
    
    let old_reader = BufReader::new(old_file);
    let new_reader = BufReader::new(new_file);
    let old_blocks = get_blocks_one_file(old_reader);
    let new_blocks = get_blocks_one_file(new_reader);
    
    let write_and_get_diff = write_one_file(&old_blocks, &new_blocks, new_path);
    if diff_in_one_file{
        save_diff(new_path, &write_and_get_diff.unwrap())?;
    }
    todo!();
    // Ok(())
}

fn get_blocks_one_file(reader: BufReader<File>) -> Vec<Vec<String>> {//io::Result<Vec<Vec<String>>>
    // let file = File::open(path);
    // let reader = BufReader::new(file);
    let mut blocks = Vec::new();
    let mut block_actual = Vec::new();
    let mut in_translate_block = false;
    // let mut in_old_new_block = false;

    for line in reader.lines() {
        let _line = line.unwrap_or_default();
        
        if is_translation_line(&_line) {
            if !block_actual.is_empty() {
                blocks.push(block_actual);
            }
            block_actual = Vec::new();
            in_translate_block = true;
            
        } else if in_translate_block && is_next_key_line(&_line) {
            in_translate_block = false;
        }
        
        block_actual.push(_line);
        
        if !in_translate_block && !block_actual.is_empty() {
            blocks.push(block_actual);
            block_actual = Vec::new();
        }
    }
    
    if !block_actual.is_empty() {
        blocks.push(block_actual);
    }
    
    blocks
}

fn write_one_file(old_blocks: &[Vec<String>], new_blocks: &[Vec<String>], output_path: &str) -> io::Result<Vec<String>> {
    let mut  index_old = 0;
    let mut index_new = 0;
    
    let mut temp_file_path = PathBuf::from(output_path);
    temp_file_path.set_extension("tmp");
    
    let temp_file = File::create(&temp_file_path)?;
    let mut temp_writer = BufWriter::new(temp_file);
    
    
    let mut diff = Vec::new();
    // Ok(diff)
    
    // diff.push(format!("Line {} (old) vs line {} (new): '{}' != '{}'",
    //     linea_original_num, linea_nuevo_num, old_key, new_key));

    while index_new < new_blocks.len() {
    let new_block = &new_blocks[index_new];
    
    if index_old < old_blocks.len() {
        let old_block = &old_blocks[index_old];
        
        let new_key = extraer_clave(&new_block[0]);
        let old_key = extraer_clave(&old_block[0]);    
        if new_key == old_key {
            // COINCIDEN: Escribir bloque ORIGINAL (con traducción)
            for linea in old_block {
                writeln!(temp_writer, "{}", linea)?;
            }
            index_old += 1;
        } else {
            // NO COINCIDEN: Escribir bloque NUEVO (sin cambios)
            for linea in new_block {
                writeln!(temp_writer, "{}", linea)?;
            }
            diff.push(format!("differents: {} != {}", old_key, new_key));
        }
         
        
    } else {
        // Se acabaron los bloques originales, copiar resto del NUEVO
        for linea in new_block {
            writeln!(temp_writer, "{}", linea)?;
        }
    }
    
        index_new += 1;
    }
    Ok(diff)

}

fn extraer_clave(linea: &str) -> String {
    let partes: Vec<&str> = linea.split_whitespace().collect();
    if partes.len() >= 3 {
        partes[2].trim_end_matches(':').to_string()
    } else {
        String::new()
    }
}

fn save_diff(new_path: &str, diff: &Vec<String>) -> io::Result<()> {
    if diff.is_empty() {
        return Ok(());
    }
    
    let mut output_path = PathBuf::from(new_path);
    output_path.set_extension("txt");
    
    let output_file = File::create(&output_path)?;
    let mut writer = BufWriter::new(output_file);
    
    for discrepancia in diff {
        writeln!(writer, "{}", discrepancia)?;
    }
    
    writer.flush()?;
    println!("diff: {}", output_path.display());
    
    Ok(())
}

fn is_translation_line(line: &str) ->bool {
    !line.starts_with('#') && !line.starts_with("translate")
}
fn is_next_key_line(line: &str) -> bool {
    // line.trim_star_matches(' ');
    line.starts_with("translate ") ||
    line.starts_with("# game/")
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
