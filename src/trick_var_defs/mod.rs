use byteorder::ReadBytesExt;
use byteorder::{ByteOrder, LittleEndian};
use std::collections::BTreeMap;
use std::io::Read;
use std::io::BufRead;

use uom::fmt::DisplayStyle::Abbreviation;
use uom::si::f64::*;
use uom::si::length::{centimeter, kilometer, meter};
use uom::si::time::second;
use uom::si::velocity::{meter_per_second};

pub struct InputArgs {
    pub name: String,
    pub path: String,
}

pub struct TrickVarDefs {
    pub defs: BTreeMap<u32, String>,
}

impl Default for TrickVarDefs {
    fn default() -> Self {
        let mut defs  = BTreeMap::new();
        defs.insert(1, "char".into());
        defs.insert(2, "unsigned char".into());
        defs.insert(4, "short".into());
        defs.insert(5, "unsigned short".into());
        defs.insert(6, "int".into());
        defs.insert(7, "unsigned int".into());
        defs.insert(8, "long".into());
        defs.insert(9, "unsigned long".into());
        defs.insert(10, "float".into());
        defs.insert(11, "double".into());
        defs.insert(12, "bit field".into());
        defs.insert(13, "unsigned bit field".into());
        defs.insert(14, "long long".into());
        defs.insert(15, "unsigned long long".into());
        defs.insert(17, "bool".into());

        TrickVarDefs {
            defs,
        }
    }
}




pub struct TrickVar {
    ident: u32,
    c_type: String
}

#[derive(Debug, Clone)]
pub struct VariableDescriptor {
    pub namelen: u32,
    pub name: String,
    pub unitlen: u32,
    pub unit: String,
    pub var_type_ident: u32,
    pub var_type: String,
    pub size_of_type: u32
}

#[derive(Debug)]
pub struct DataRecord {
    var_type: String,
    var_size: u32,
    var_value: dyn std::any::Any,
}

impl Default for VariableDescriptor {
    fn default() -> VariableDescriptor {
        VariableDescriptor { 
            namelen: 1 , 
            name: " ".into(), 
            unitlen: 1, 
            unit: "".into(), 
            var_type_ident: 1,
            var_type: "".into(), 
            size_of_type: 1
        }
    }
}

pub fn c_string(bytes: &[u8]) -> Option<&str> {
    let bytes_without_null = match bytes.iter().position(|&b| b == 0) {
        Some(ix) => &bytes[..ix],
        None => bytes,
    };

    std::str::from_utf8(bytes_without_null).ok()
}

//char..sometimes
pub fn i8_from_bytes(bytes: &[u8]) -> Option<i8> {
    let mut rdr = std::io::Cursor::new(bytes);

   Some(rdr.read_i8().unwrap())

}

//unsigned int
pub fn u32_from_bytes(bytes: &[u8]) -> Option<u32> {
    let mut rdr = std::io::Cursor::new(bytes);

    rdr.read_u32::<LittleEndian>().ok()
}

//long int
pub fn u64_from_bytes(bytes: &[u8]) -> Option<u64> {
    let mut rdr = std::io::Cursor::new(bytes);

    rdr.read_u64::<LittleEndian>().ok()

}

//double
pub fn f64_from_bytes(bytes: &[u8]) -> Option<f64> {
    let mut rdr = std::io::Cursor::new(bytes);

    rdr.read_f64::<LittleEndian>().ok()
    
}

//float
pub fn f32_from_bytes(bytes: &[u8]) -> Option<f32> {
    let mut rdr = std::io::Cursor::new(bytes);

    rdr.read_f32::<LittleEndian>().ok()
}

//char (sometimes)
pub fn i32_from_bytes(bytes: &[u8]) -> Option<i32> {
    let mut rdr = std::io::Cursor::new(bytes);

   rdr.read_i32::<LittleEndian>().ok()
   
}

pub fn i16_from_bytes(bytes: &[u8]) -> Option<i16> {
    let mut rdr = std::io::Cursor::new(bytes);

    rdr.read_i16::<LittleEndian>().ok()
}

#[derive(Debug, Clone)]
pub struct LogFileInfo {
    pub header_file_name: String,
    pub log_file_name: String,
    pub full_path: String,
}

impl Default for LogFileInfo {
    fn default() -> Self {
        LogFileInfo {
            header_file_name: "".into(),
            log_file_name: "".into(),
            full_path: "".into()
        }
    }
}

#[derive(Debug, Clone)]
pub struct TrickData {
    pub log_file: LogFileInfo,
    pub descriptors: Vec<VariableDescriptor>,
    pub data: Vec<TrickColumn>,
    pub num_params: u32,
}

#[derive(Debug, Clone)]
pub struct TrickColumn {
    pub data: Vec<f64>
}


impl Default for TrickColumn {
    fn default() -> Self {
        TrickColumn { 
            data: vec![0.0; 0]
        }
    }
}

impl Default for TrickData {
    fn default() -> Self {
        TrickData {
            log_file: LogFileInfo { 
                ..Default::default()
             },
             descriptors: vec![VariableDescriptor::default(); 0],
             data: vec![TrickColumn::default(); 0],
             num_params: 0,
        }
    }  
}


impl TrickData {

    pub fn new(input: InputArgs) -> Self {
    
        let mut path: String = input.path.to_owned();
        path.push_str("/");
        
        let mut header_file_name = input.name.to_owned();
        let header_file_type = ".header";
        
        header_file_name += header_file_type;

        let mut log_file_name = input.name;
        let log_binary_file_type = ".trk";
        log_file_name += log_binary_file_type;

        let name:  String = header_file_name.to_owned();
        path.push_str(&log_file_name);

        TrickData {
            log_file: LogFileInfo {
                header_file_name,
                log_file_name,
                full_path: path,
            },
            ..Default::default()
        }

        //println!("{}, {}", trick_data.log_file.header_file_name, trick_data.log_file.log_file_name);
        //println!("{}", trick_data.log_file.full_path);

    }

    pub fn read(&mut self) {
        let trick_type_defs= TrickVarDefs::default();

        let file_buf = std::fs::read(&self.log_file.full_path).unwrap();

        let mut rdr = std::io::Cursor::new(file_buf);

        let descriptors = read_descriptors(&mut rdr);
        
        let trick_type_defs = TrickVarDefs::default();

         //Okay so we have n (length of descriptors vec) rows we need to break into columns to fit in the data struct...
        let mut data: Vec<TrickColumn> = vec![TrickColumn {
            data: vec![0.0; 0],
        }; descriptors.len()];


        //Kind of assuming these are all doubles right now...
        while rdr.has_data_left().unwrap() {
            let row = read_row(&mut rdr, &descriptors, &trick_type_defs);
            
            //take each column of the row and stick it into the approriate data column
            for i in 0..descriptors.len() {
                data[i].data.push(row[i]);
            }
        }
        
       self.data = data.clone();
       self.descriptors = descriptors.clone();
       
    }   

}

fn read_descriptors(rdr: &mut std::io::Cursor<Vec<u8>>) -> Vec<VariableDescriptor>{
        
    let mut trick_ver_buffer: Vec<u8> = vec![0; 10];

    rdr.read_exact(&mut trick_ver_buffer);

    //print!("{:?}\n", c_string(&trick_ver_buffer));

    let mut num_params_buffer: Vec<u8> = vec![0; 4];
    
    rdr.read_exact(&mut num_params_buffer);

    //print!("{:?}\n", num_params_buffer);

    let num_params = u32_from_bytes(&num_params_buffer).unwrap();

    let trick_type_defs = TrickVarDefs::default();

    let mut descriptors = vec![VariableDescriptor::default(); 0];

    for _ in 0..num_params {
        descriptors.push(read_variable_descriptor(rdr, &trick_type_defs));
    }

    descriptors

}


fn read_variable_descriptor(rdr: &mut std::io::Cursor<Vec<u8>>,  trick_type_defs: &TrickVarDefs) ->  VariableDescriptor {

    //Read the Time Variable Descriptor
    let mut name_length_buffer: [u8; 4] = core::array::from_fn(|i| i as u8);

    rdr.read_exact(&mut name_length_buffer);
    //print!("{:?}\n", name_length_buffer);

    let name_length = u32_from_bytes(&name_length_buffer).unwrap();

    let mut name_buffer: Vec<u8> = vec![0; name_length as usize];

    rdr.read_exact(&mut name_buffer);

   // print!("{:?}\n", c_string(&name_buffer));

    let mut unit_length_buffer: Vec<u8> = vec![0; 4];
    rdr.read_exact(&mut unit_length_buffer);
    //print!("{:?}\n", unit_length_buffer);

    let unit_length =  u32_from_bytes(&unit_length_buffer).unwrap();

    let mut unit_buffer: Vec<u8> = vec![0; unit_length as usize];

    rdr.read_exact(&mut unit_buffer);
    //print!("{:?}\n", c_string(&unit_buffer));

    let mut unit_type_buffer: Vec<u8> = vec![0; 4];
    
    rdr.read_exact(&mut unit_type_buffer);
    //print!("{:?}\n", unit_type_buffer);
    let unit_type =  u32_from_bytes(&unit_type_buffer).unwrap();

    let mut trick_type: String = String::from("");
    
    if trick_type_defs.defs.contains_key(&unit_type) {
        trick_type = String::from(trick_type_defs.defs.get(&unit_type).unwrap());
    }

    let mut type_size_buffer: Vec<u8> = vec![0;4];
    rdr.read_exact(&mut type_size_buffer);
    let type_size = u32_from_bytes(&type_size_buffer).unwrap();

    VariableDescriptor {
        namelen: name_length,
        name: c_string(&name_buffer).unwrap().into(),
        unitlen: unit_length,
        unit: c_string(&unit_buffer).unwrap().into(),
        var_type_ident: unit_type,
        var_type: trick_type,
        size_of_type: type_size
    }



}

//Looks like variables are stored in rows..which is a little akward if the rows have different
    //primitive types
    fn read_row(rdr: &mut std::io::Cursor<Vec<u8>>, descriptors: &Vec<VariableDescriptor>,trick_type_defs: &TrickVarDefs) -> Vec<f64>  {

        //default buffer to u8 64 bits
        let mut buffer: Vec<u8> = vec![0; 8];
        let mut results: Vec<f64> = vec![0.0; 0];   

        let num_rows = descriptors.len();
        
        //Type conversions based on 
        // https://locka99.gitbooks.io/a-guide-to-porting-c-to-rust/content/features_of_rust/types.html

        let mut char_vec: Vec<i8> = vec![0; 0];
        let mut uchar_vec: Vec<u8> = vec![0; 0];
        let mut short_vec: Vec<i16> = vec![0; 0];
        let mut uint_short_vec: Vec<u16> = vec![0; 0];
        let mut int_vec: Vec<i32> = vec![0; 0];
        let mut uint_vec: Vec<u32> = vec![0; 0];
        let mut long_vec: Vec<i64> = vec![0; 0];
        let mut long_long_vec: Vec<i64> = vec![0; 0];
        let mut float_vec: Vec<f32> = vec![0.0; 0];
        let mut double_vec: Vec<f32> = vec![0.0; 0];
        let mut bool_vec: Vec<bool> = vec![false; 0];

        for desc in descriptors.iter().take(num_rows) {
            
            let mut variable_type: String = "".into();
            let trick_type = trick_type_defs.defs.get(&desc.var_type_ident).unwrap();
            
            match trick_type.as_str() {
                "char" => {
                    buffer.resize(8, 0);
                    rdr.read_exact(&mut buffer);
                    let mut val = i8_from_bytes(&buffer).unwrap();
                    char_vec.push(val);
                },
                "unsigned char" => {
                    buffer.resize(8, 0);
                    rdr.read_exact(&mut buffer);
                    let mut val = i8_from_bytes(&buffer).unwrap();
                    char_vec.push(val);
                },
                "short" => {
                    todo!();
                },
                "unsigned short" => {
                    todo!();
                },
                "int" => {
                    todo!();
                },
                "unsigned int" => {
                    todo!();
                },
                "long" => {
                    todo!();
                },
                "unsigned long" => {
                    todo!();
                },
                "float" => {
                    buffer.resize(32, 0);
                    rdr.read_exact(&mut buffer);
                    let mut val = f32_from_bytes(&buffer).unwrap();
                    float_vec.push(val);
                },
                "double" => {
                    rdr.read_exact(&mut buffer);
                    let mut val = f64_from_bytes(&buffer).unwrap();
                    results.push(val);           
                },
                "bit field" => {
                    todo!();
                },
                "unsigned bit field" => {
                    todo!();
                },
                "long long" => {
                    todo!();
                },
                "unsigned long long" => {
                    todo!();
                },
                "bool" => {
                    todo!();
                }
                _ => {
                    todo!();
                }

            }
        }

        results
}



#[test]
fn test_read_descriptors() 
{
    let var_descriptors = vec![VariableDescriptor::default(); 0];

    let file_buf = std::fs::read("logs/log_cannon.trk").unwrap();

    let mut rdr = std::io::Cursor::new(file_buf);

    let descriptors = read_descriptors(&mut rdr);
    
    assert_eq!(descriptors.len(), 3);
    
    assert_eq!(descriptors[0].namelen, 17);
    assert_eq!(descriptors[0].name, "sys.exec.out.time");
    assert_eq!(descriptors[0].unitlen, 1);
    assert_eq!(descriptors[0].unit, "s");
    assert_eq!(descriptors[0].var_type_ident, 11);

    assert_eq!(descriptors[1].namelen, 17);
    assert_eq!(descriptors[1].name, "dyn.cannon.pos[0]");
    assert_eq!(descriptors[1].unitlen, 1);
    assert_eq!(descriptors[1].unit, "m");
    assert_eq!(descriptors[1].var_type_ident, 11);

    assert_eq!(descriptors[2].namelen, 17);
    assert_eq!(descriptors[2].name, "dyn.cannon.pos[1]");
    assert_eq!(descriptors[2].unitlen, 1);
    assert_eq!(descriptors[2].unit, "m");
    assert_eq!(descriptors[2].var_type_ident, 11);
}