pub mod output_debug_string;
mod read_incoming_data;
mod write_outgoing_data;

pub unsafe fn load() -> anyhow::Result<()> {
    read_incoming_data::hook()?;
    write_outgoing_data::hook()?;
    output_debug_string::hook()?;

    Ok(())
}

pub unsafe fn unload() -> anyhow::Result<()> {
    read_incoming_data::unhook()?;
    write_outgoing_data::unhook()?;
    output_debug_string::unhook()?;

    Ok(())
}
