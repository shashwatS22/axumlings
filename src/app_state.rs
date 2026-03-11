use anyhow::{bail, Context, Result};
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, Write};
use std::path::Path;

use crate::exercise::Exercise;
use crate::info_file::ExerciseInfo;

const STATE_FILE_NAME: &str = ".axumlings-state.txt";
const STATE_FILE_HEADER: &[u8] = b"DON'T EDIT THIS FILE!\n\n";

#[derive(Clone, Copy)]
pub enum StateFileStatus {
    Read,
    NotRead,
}

pub struct AppState {
    current_exercise_ind: usize,
    exercises: Vec<Exercise>,
    n_done: u16,
    final_message: String,
    state_file: File,
    file_buf: Vec<u8>,
}

impl AppState {
    pub fn new(
        exercise_infos: Vec<ExerciseInfo>,
        final_message: String,
    ) -> Result<(Self, StateFileStatus)> {
        let mut state_file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(STATE_FILE_NAME)
            .with_context(|| format!("Failed to open or create {}", STATE_FILE_NAME))?;

        let mut exercises: Vec<Exercise> = exercise_infos
            .into_iter()
            .map(|info| Exercise {
                name: info.name.clone(),
                dir: info.dir.clone(),
                path: info.path(),
                test: info.test,
                strict_clippy: info.strict_clippy,
                hint: info.hint,
                done: false,
            })
            .collect();

        let mut current_exercise_ind = 0;
        let mut n_done = 0;
        let mut file_buf = Vec::with_capacity(2048);

        let state_file_status = {
            if state_file.read_to_end(&mut file_buf).is_err() {
                StateFileStatus::NotRead
            } else {
                let mut lines = file_buf.split(|&c| c == b'\n').skip(2);

                let current_exercise_name = match lines.next() {
                    Some(name) if !name.is_empty() => name,
                    _ => {
                        file_buf.clear();
                        file_buf.extend_from_slice(STATE_FILE_HEADER);
                        return Ok((
                            Self {
                                current_exercise_ind: 0,
                                exercises,
                                n_done: 0,
                                final_message,
                                state_file,
                                file_buf,
                            },
                            StateFileStatus::NotRead,
                        ));
                    }
                };

                let _ = lines.next();

                let mut done_exercises = HashSet::new();
                for name in lines {
                    if name.is_empty() {
                        break;
                    }
                    done_exercises.insert(name);
                }

                for (ind, ex) in exercises.iter_mut().enumerate() {
                    if done_exercises.contains(ex.name.as_bytes()) {
                        ex.done = true;
                        n_done += 1;
                    }
                    if ex.name.as_bytes() == current_exercise_name {
                        current_exercise_ind = ind;
                    }
                }

                StateFileStatus::Read
            }
        };

        file_buf.clear();
        file_buf.extend_from_slice(STATE_FILE_HEADER);

        Ok((
            Self {
                current_exercise_ind,
                exercises,
                n_done,
                final_message,
                state_file,
                file_buf,
            },
            state_file_status,
        ))
    }

    pub fn current_exercise_ind(&self) -> usize {
        self.current_exercise_ind
    }

    pub fn exercises(&self) -> &[Exercise] {
        &self.exercises
    }

    pub fn n_done(&self) -> u16 {
        self.n_done
    }

    pub fn n_pending(&self) -> u16 {
        self.exercises.len() as u16 - self.n_done
    }

    pub fn current_exercise(&self) -> &Exercise {
        &self.exercises[self.current_exercise_ind]
    }

    pub fn current_exercise_mut(&mut self) -> &mut Exercise {
        &mut self.exercises[self.current_exercise_ind]
    }

    fn write(&mut self) -> Result<()> {
        self.file_buf.truncate(STATE_FILE_HEADER.len());
        let name = self.exercises[self.current_exercise_ind].name.clone();
        self.file_buf.extend_from_slice(name.as_bytes());
        self.file_buf.push(b'\n');

        for ex in &self.exercises {
            if ex.done {
                self.file_buf.push(b'\n');
                self.file_buf.extend_from_slice(ex.name.as_bytes());
            }
        }

        self.state_file.rewind()?;
        self.state_file.set_len(0)?;
        self.state_file.write_all(&self.file_buf)?;

        Ok(())
    }

    pub fn set_current_exercise_ind(&mut self, ind: usize) -> Result<()> {
        if ind >= self.exercises.len() {
            bail!("Exercise index out of bounds");
        }
        self.current_exercise_ind = ind;
        self.write()
    }

    pub fn set_current_exercise_by_name(&mut self, name: &str) -> Result<()> {
        let ind = self
            .exercises
            .iter()
            .position(|e| e.name == name)
            .with_context(|| format!("Exercise '{}' not found", name))?;
        self.set_current_exercise_ind(ind)
    }

    pub fn set_pending(&mut self, exercise_ind: usize) -> Result<()> {
        if let Some(ex) = self.exercises.get_mut(exercise_ind) {
            if ex.done {
                ex.done = false;
                self.n_done -= 1;
                self.write()?;
            }
        }
        Ok(())
    }

    pub fn set_done(&mut self, exercise_ind: usize) -> Result<()> {
        if let Some(ex) = self.exercises.get_mut(exercise_ind) {
            if !ex.done {
                ex.done = true;
                self.n_done += 1;
                self.write()?;
            }
        }
        Ok(())
    }

    pub fn reset_current_exercise(&mut self) -> Result<String> {
        let path = self.current_exercise().path.clone();
        self.set_pending(self.current_exercise_ind)?;
        crate::embedded::EMBEDDED_FILES.write_exercise_to_disk(self.current_exercise_ind, &path)?;
        Ok(path)
    }

    pub fn next_pending_exercise_ind(&self) -> Option<usize> {
        let next_ind = self.current_exercise_ind + 1;
        self.exercises
            .get(next_ind..)
            .and_then(|slice| slice.iter().position(|e| !e.done).map(|i| next_ind + i))
            .or_else(|| {
                self.exercises[..self.current_exercise_ind]
                    .iter()
                    .position(|e| !e.done)
            })
    }

    pub fn current_solution_path(&self) -> Result<Option<String>> {
        let path = self.current_exercise().sol_path();
        if Path::new(&path).exists() {
            return Ok(Some(path));
        }
        let path = crate::embedded::EMBEDDED_FILES
            .write_solution_to_disk(self.current_exercise_ind, &self.current_exercise().name)?;
        Ok(Some(path))
    }
}
