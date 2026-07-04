use anyhow::Error;

pub enum VError
{
    FILE_READ_ERROR{ e: Error },
}