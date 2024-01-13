pub(crate) mod packet;
mod pcap;
pub(crate) mod writer;

use self::{pcap::Pcap, writer::Writer};
use pcap_file::pcapng::{blocks::interface_description::InterfaceDescriptionBlock, PcapNgBlock, PcapNgWriter};
use std::path::PathBuf;

pub fn setup_capture(file_path: Option<PathBuf>) {
    if let Some(file_path) = file_path {
        let file = std::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(file_path.with_extension("pcap"))
            .unwrap();

        let mut pcap_writer = PcapNgWriter::new(file).unwrap();

        // Write headers
        let _ = pcap_writer.write_block(
            &InterfaceDescriptionBlock {
                linktype: pcap_file::DataLink::ETHERNET,
                snaplen: 0xFFFF,
                options: vec![],
            }
            .into_block(),
        );

        let writer = Box::new(Pcap::new(pcap_writer));
        attach(writer)
    } else {
        // If no file path is provided
        // Do nothing
    }
}

/// Attaches a writer to the capture module.
///
/// # Errors
/// Returns an Error if the writer is already set.
fn attach(writer: Box<dyn Writer + Send + Sync>) {
    crate::socket::capture::set_writer(writer);
}
