use std::path::PathBuf;
use worldlib::serialized::SerializedNPC;
use worldlib::map::npc::{NPCManager, NPCMap};

pub mod npc_type;

pub fn load_npc_entries(npc_path: PathBuf) -> NPCManager {
    let mut npcs = NPCMap::new();
    if let Ok(dir) = std::fs::read_dir(npc_path) {
        for entry in dir {
            if let Ok(entry) = entry {
                let file = entry.path();
                let data = std::fs::read_to_string(&file).unwrap_or_else(|err| panic!("Could not get NPC file at {:?} with error {}", file, err));
                let npc_result: Result<SerializedNPC, ron::Error> = ron::from_str(&data);
                match npc_result {
                    Ok(npc) => {
                        npcs.insert(npc.id, npc.npc);
                    },
                    Err(err) => {
                        panic!("Could not parse NPC at {:?} with error {} at position {}", file, err, err.position);
                    },
                }
            }
        }
    } 
    NPCManager::new(npcs)
}