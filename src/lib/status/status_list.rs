use super::super::prelude::*;

pub struct StatusOptions;

pub struct StatusList {
    repository: &Repository,
}

impl StatusList {
    // TODO: Support options
    pub fn new(repository: &Repository, options: StatusOptions) -> Result<StatusList> {
        StatusList {
            repository: repository,
        }
    }

    pub fn entries(&self) -> Result<usize> {

    }
}

