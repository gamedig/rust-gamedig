pub(crate) mod packet;
mod pcap;
pub mod writer;

use pcap_file::pcapng::PcapNgBlock;
use writer::Writer;

use self::pcap::Pcap;

pub fn setup_capture(file_name: Option<String>) {
    if let Some(file_name) = file_name {
        let file = std::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(file_name)
            .unwrap();

        let mut pcap_writer = pcap_file::pcapng::PcapNgWriter::new(file).unwrap();

        // Write headers
        pcap_writer.write_block(
            &pcap_file::pcapng::blocks::interface_description::InterfaceDescriptionBlock {
                linktype: pcap_file::DataLink::ETHERNET,
                snaplen: 0xFFFF,
                options: vec![],
            }
            .into_block(),
        );

        let writer = Box::new(Pcap::new(pcap_writer));
        attach(writer)
    } else {
        // Do nothing
    }
}

/// Attaches a writer to the capture module.
///
/// # Errors
/// Returns an `io::Error` if the writer is already set.
fn attach(writer: Box<dyn Writer + Send + Sync>) {
    crate::socket::capture::set_writer(writer);
}
