extern crate rustache;
use rustache::{HashBuilder, Render};

extern crate rustc_serialize;
use rustc_serialize::{json , Decodable, Decoder};

extern crate walkdir;
use walkdir::{DirEntry, WalkDir, WalkDirIterator};

use std::io::BufReader;
use std::io::BufRead;
use std::io::Cursor;
use std::io::Write;

use std::fs;
use std::fs::File;

use std::collections::BTreeMap;

use std::env;


#[derive(Clone)]
struct Dep{
    name: String,
    optional: bool,
    kind: String,
    req: String,
    target: String,
}


impl Decodable for Dep {
    fn decode<D: Decoder>(d: &mut D) -> Result<Dep, D::Error> {
        d.read_struct("Dep", 5, |d| {
            let name = try!(d.read_struct_field("name", 0, |d| { d.read_str() }));
            let optional = try!(d.read_struct_field("optional", 1, |d| { 
                Ok( match d.read_bool(){
                    Ok(opt) => opt,
                    Err(_) => false,
                })}));
            let kind = try!(d.read_struct_field("kind", 2, |d| { 
                Ok( match d.read_str(){
                    Ok(opt) => opt,
                    Err(_) => "".to_string(),
                }) }));
            
            let req = try!(d.read_struct_field("req", 3, |d| { d.read_str() }));

            let target = try!(d.read_struct_field("target", 4, |d| { 
                 Ok(match d.read_str() {
                    Ok(opt) => opt,
                    Err(_) => "".to_string(),
                })}));

            let ret = Dep{name: name, optional: optional, kind: kind, target: target, req: req };
            return Ok(ret);
        })
    }
}

#[derive(RustcDecodable, Clone)]
struct MyCrate{
    name: String,
    vers: String,
    deps: Vec<Dep>,
    cksum: String,
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

//converts the version string into an array of u32s
fn convert_version(version: &str) -> Vec<u32> {
    return version.split(&['.'][..]).map(|s| match s.parse::<u32>(){
                Ok(x) => x,
                Err(_) => 0_u32,
            }).collect();
}

//Parses one single crates.io file
fn parseCrates(f: &File) -> BTreeMap<Vec<u32>,MyCrate>{
  let mut all_versions = BTreeMap::new();
  let mut reader = BufReader::new(f);
  //parse all crates
  for line in reader.lines(){
      let l = line.unwrap();

      let mut next_crate: MyCrate = match json::decode(&l){
          Ok(x) => x,
          Err(err) => { println!("ERROR while parsing a crate: {}", err); continue} ,
      };

//      if next_crate.vers.contains("-"){
          //skip beta versions
//          continue;
//      }

      //remove everything after an `+` since those vallues can be ignored for versioning
      //remove everything after an `-` those versions are unstable pre-release versions. 
      //we allow them but only ceep the latest.
      let prep_for_split = next_crate.vers.clone();
      let split: Vec<&str> = prep_for_split.split(&['+', '-'][..]).collect();
      let v: &str = split[0];
      next_crate.vers = v.to_string();

      let version = convert_version(&next_crate.vers);
      //insert the latest version, discard the previous value (if there is one)
      all_versions.insert(version, next_crate);
  }
  return all_versions;
}

//convert a vector of deps into a string and resolve the given versions.
fn create_dep_string(deps: &Vec<Dep>) -> String{
  let mut dep_str = "".to_string();
  for d in deps {
      //FIXME this breaks things for windows ans macos
      if !d.optional && d.kind != "dev" && !d.target.contains("windows") && !d.target.contains("macos"){
          if d.req.contains("<") || d.req.contains("=") || d.req.contains(">") || d.req.contains("*"){
              //Cant resolve use newest version
              dep_str = dep_str + " " + &(d.name);
              continue;
          }
          let mut x: Vec<&str> = d.req.split(".").collect();

          if x.len() > 3 {
              //Cant resolve use newest version
              dep_str = dep_str + " " + &(d.name);
              continue;
          }
          if d.req.starts_with("~") {
              if x.len() == 1 {
                  dep_str = dep_str + " all__" + &(d.name) + "." +  &(d.name) + "_" + x[0].trim_left_matches("~");
              }else {
                  dep_str = dep_str + " all__" + &(d.name) + "." +  &(d.name) + "_" + x[0].trim_left_matches("~") + "_" + x[1];
              }
          }else if d.req.starts_with("^") {
              dep_str = dep_str + " all__" + &(d.name) + "." +  &(d.name) + "_" + x[0].trim_left_matches("^");
              x.remove(0);
              for i in x {
                  dep_str = dep_str + "_" + i;
                  if i != "0" {
                      break;
                  }
              }

          }else {
              if x.len() > 3 {
                  //Cant resolve use newest version
                  dep_str = dep_str + " " + &(d.name);
              }else{
                  dep_str = dep_str + " all__" + &(d.name) + "." +  &(d.name);
                  for i in x {
                      dep_str = dep_str + "_" + i;
                  }
              }
          }
      }
  }
  return dep_str;
}


fn main() {
    //check arguments
    let args: Vec<_> = env::args().collect();
    if args.len() < 3 {
        println!("The first argument should be the path of the crates.io-index");
        println!("The second argument should be the path for the nixcrates index");
        return;
    }else{
        println!("Inputh path is {}", args[1]);
        println!("Output path is {}", args[2]);
    }

    let input = &args[1];
    let output = &args[2];

    //template for the nix file
    let template = r#"
  {{package_name}} = buildCratesLib {
    name = "{{name}}";
    version = "{{vers}}";
    hash = "{{cksum}}";
    deps = with allCrates; [ {{deps}} ];
  };"#;

    let packages_path = output.to_string() + "/generated-crates.nix";
    let mut packages = File::create(packages_path).unwrap();
    write!(packages, "#DON'T EDIT. AUTOGENERATED FILE");
    write!(packages, "\n{{");
    write!(packages, "\n  allCrates = self: super: rec {{");

    //traverse through the crates.io index
    for entry in WalkDir::new(input).into_iter().filter_entry(|e| !is_hidden(e)) {
        let entry = entry.unwrap();
        if entry.file_type().is_dir(){
            //create the equivalent folder for the nix index
            let new_path = "".to_string() + output + &(entry.path().to_str().unwrap().trim_left_matches(input));
            fs::create_dir_all(new_path);
        }else if entry.file_type().is_file(){
            //create the equivalent nix file for the nix index

            //check if the file is the config.json
            if entry.path().to_str().unwrap().ends_with("config.json") ||
               entry.path().to_str().unwrap().contains(".git") ||
               entry.path().to_str().unwrap().ends_with(".nix"){
                continue;
            }
            //Open file
            let f = match File::open(entry.path()){
                Ok(x) => x,
                Err(_) => continue,
            };
            println!("{}", entry.path().to_str().unwrap());


            //btree is used to store all versions (sorted).
            let mut all_versions = parseCrates(&f);

            if all_versions.len() == 0{
                println!("WARNING: empty package");
                continue;
            }

            let new_sub_path = (entry.path().to_str().unwrap().trim_left_matches(input)).to_string() + ".nix";
            let new_path = output.to_string() + &new_sub_path;

            let mut buffer = File::create(new_path).unwrap();

            write!(buffer, "#DON'T EDIT. AUTOGENERATED FILE");
            write!(buffer, "\n{{buildCratesLib, allCrates}}:");
            write!(buffer, "\nrec {{");

            let (first_key, first_value) = all_versions.iter().next().unwrap();
            let mut prev = first_value.clone();
            let mut prev_version = first_key.clone();
            let mut prev_path = new_sub_path.clone();

            let name = first_value.name.clone();
            write!(packages, "\n    \"all__{}\" = self.callPackage ./{} {};", name , new_sub_path, "{ }");
            for (version, c) in &all_versions {
                let next_crate = c.clone();
                //create a string containing all deps
                let dep_str = create_dep_string(&next_crate.deps);

                let full_version = "_".to_string() + &next_crate.vers.replace(".", "_");
                let package_name = next_crate.name.clone() + &full_version;
                let data = HashBuilder::new()
                    .insert("package_name", package_name.clone())
                    .insert("name", next_crate.name.clone())
                    .insert("vers", next_crate.vers)
                    .insert("cksum", next_crate.cksum)
                    .insert("deps", dep_str);

                //write nix file
                let mut rv = Cursor::new(Vec::new());
                data.render(template, &mut rv).unwrap();
                let res = String::from_utf8(rv.into_inner()).unwrap();

                write!(buffer, "{}", res);
                //add entry to the generated-crates.nix
//                write!(packages, "\n\"{}\" = all__{}.\"{}\";",  package_name, name, package_name);

                let full_version = "_".to_string() + &prev_version[0].to_string() + "_" + &prev_version[1].to_string() + "_" + &prev_version[2].to_string();

                if prev_version[0] < version[0] {
                    let smal_version = "_".to_string() + &prev_version[0].to_string() + "_" + &prev_version[1].to_string();
                    let package_name = prev.name.clone() + &smal_version;
//                    write!(packages, "\n\"{}\" = all__{}.\"{}\";",  package_name, name, prev.name.clone() + &full_version);
                    write!(buffer, "\n  \"{}\" = {};",  package_name, prev.name.clone() + &full_version);

                    let smal_version = "_".to_string() + &prev_version[0].to_string();
                    let package_name = prev.name.clone() + &smal_version;
//                    write!(packages, "\n\"{}\" = all__{}.\"{}\";",  package_name, name, prev.name.clone() + &full_version);
                    write!(buffer, "\n  \"{}\" = {};",  package_name, prev.name.clone() + &full_version);
                }else if prev_version[1] < version[1] {
                    let smal_version = "_".to_string() + &prev_version[0].to_string() + "_" + &prev_version[1].to_string();
                    let package_name = prev.name.clone() + &smal_version;
//                    write!(packages, "\n\"{}\" = all__{}.\"{}\";",  package_name, name, prev.name.clone() + &full_version);
                    write!(buffer, "\n  \"{}\" = {};",  package_name, prev.name.clone() + &full_version);
                }


                prev_version = version.clone();
                prev = c.clone();
            }
            //add more versions to the generated-crates.nix file
            let full_version = "_".to_string() + &prev_version[0].to_string() + "_" + &prev_version[1].to_string() + "_" + &prev_version[2].to_string();

            let smal_version = "_".to_string() + &prev_version[0].to_string() + "_" + &prev_version[1].to_string();
            let package_name = prev.name.clone() + &smal_version;
//            write!(packages, "\n\"{}\" = all__{}.\"{}\";",  package_name, name, prev.name.clone() + &full_version);
            write!(buffer, "\n  \"{}\" = {};",  package_name, prev.name.clone() + &full_version);
            let smal_version = "_".to_string() + &prev_version[0].to_string();
            let package_name = prev.name.clone() + &smal_version;
//            write!(packages, "\n\"{}\" = all__{}.\"{}\";",  package_name, name, prev.name.clone() + &full_version);
            write!(buffer, "\n  \"{}\" = {};",  package_name, prev.name.clone() + &full_version);

            write!(packages, "\n    \"{}\" = all__{}.\"{}\";",  prev.name.clone(), name, prev.name.clone() + &full_version);


            //closing brace for the crates - nix expression
            write!(buffer, "}}");

        }
    }
    // closing braces for teh generated crates.nix file
    write!(packages, "\n }};");
    write!(packages, "\n }}");
}
