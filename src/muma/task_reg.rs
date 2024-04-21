use crate::muma::{Id, Task};
use crate::muma::error::{MumaResult, MumaError};
use once_cell::sync::OnceCell;
use std::sync::{Mutex, MutexGuard};
use std::collections::btree_map::BTreeMap;
use serde::{Serialize, Deserialize};
use bimap::BiBTreeMap as BiMap;
use std::io::Write;
use std::path::{Path, PathBuf};


static INSTANCE: OnceCell<Mutex<TaskReg>> = OnceCell::new();


#[derive(Default, Debug, Serialize, Deserialize)]
pub struct TaskReg
{
    top_level: BTreeMap<Id, Task>,
    child_level: BTreeMap<Id, Task>,

    #[serde(skip_serializing, skip_deserializing)]
    human_id: BiMap<String, Id>,
}


impl TaskReg
{
    pub fn default_path() -> MumaResult<PathBuf>
    {
        let default = PathBuf::from(std::env::var("HOME").expect("This Should be set")).canonicalize()?.join(".config/muma/todo.toml");

        if !default.exists()
        {
            std::fs::create_dir_all(default.parent().unwrap())?;            
            TaskReg::default().save(&default)?;        
        }
        
        Ok(default)
    }

    pub fn save(&self, path: impl AsRef<Path>) -> MumaResult<()>
    {
        let data = toml::to_string(self)?;

        let mut file = std::fs::File::options()
                .write(true)
                .create(true)
                .truncate(true)
                .open(path)?;
    
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    pub fn read(path: impl AsRef<Path>) -> MumaResult<Self>
    {
        let data = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&data)?)
    }

    pub fn init() -> MumaResult<()>
    {
        let instance = Self::read(Self::default_path()?)?;
        INSTANCE.set(Mutex::new(instance)).expect("TaskReg can only be initialized once");
        Self::generate_human_id_map();
        Ok(())
    }

    pub fn get() -> MutexGuard<'static, TaskReg>
    {
        INSTANCE.get()
            .expect("TaskReg was never initialized")
            .lock()
            .unwrap()
    }

    pub fn exists(task: &Id) -> bool
    {
        let me = Self::get();
        me.top_level.get(task).is_some() || me.child_level.get(task).is_some()
    }

    pub fn add_task(task: Task) -> MumaResult<()>
    {
        if let Some(parent) = &task.parent
        {
            match Self::get().get_task_mut(parent)
            {
                Some(t) => (t.children.push(task.uuid)), 
                None => return Err(MumaError::TaskDoesNotExist(*parent)),
            }

            Self::get().child_level.insert(task.uuid, task);
        }

        else
        {
            Self::get().top_level.insert(task.uuid, task);
        }

        Ok(())
    }

    pub fn get_task_mut(&mut self, id: &Id) -> Option<&mut Task>
    {
        if let Some(task) = self.top_level.get_mut(id)
        {
            return Some(task);
        }

        if let Some(task) = self.child_level.get_mut(id)
        {
            return Some(task);
        }

        None
    }

    pub fn copy_hid() -> BiMap<String, Id>
    {
        Self::get().human_id.clone()
    }

    
    pub fn remove_task(id: &Id) -> MumaResult<()>
    {   
        let mut children = match Self::get().get_task_mut(id)
        {
            Some(task) => task.children.drain(..).collect::<Vec<_>>(),
            None => return Err(MumaError::TaskDoesNotExist(*id)),
        };

        while let Some(child) = children.pop()
        {
            Self::remove_task(&child)?;
        }

        let mut me = Self::get();
        
        if me.top_level.remove(id).is_none()
        {
            me.child_level.remove(id);
        }

        Ok(())
    }

    pub fn set_done(id: &Id, done: bool) -> MumaResult<()>
    {
        let mut children = match Self::get().get_task_mut(id)
        {
            Some(task) => task.children.clone(),
            None => return Err(MumaError::TaskDoesNotExist(*id)),
        };

        while let Some(child) = children.pop()
        {
            Self::set_done(&child, done)?;
        }

        Self::get().get_task_mut(id).unwrap().done = done;

        Ok(())
    }

    pub fn hid2id(hid: &str) -> Option<Id>
    {
        Self::get().human_id.get_by_left(hid).copied()
    }

    pub fn id2hid(id: &Id) -> Option<String>
    {
        Self::get().human_id.get_by_right(id).map(|e| e.to_string())
    }

    pub fn generate_human_id_map()
    {
        let mut me = Self::get();
        let mut human_id = BiMap::new();
        let mut counter = 1;

        for (key, val) in me.top_level.iter()
        {
            let id = counter.to_string();
            human_id.insert(id.clone(), *key);
            me.generate_human_id_map_rec_impl(&mut human_id, val, &id);
            counter += 1;
        }

        me.human_id = human_id;
    }

    fn generate_human_id_map_rec_impl(&self, map: &mut BiMap<String, Id>, task: &Task, id_prefix: &str)
    {
        let mut counter = 1;

        for key in &task.children
        {
            let val = match self.child_level.get(key)
            {
                Some(val) => val,
                None => continue,
            };

            let id = format!("{id_prefix}.{counter}");
            map.insert(id.clone(), *key);
            self.generate_human_id_map_rec_impl(map, val, &id);
            counter += 1;
        }
    }
}
