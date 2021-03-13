pub trait DataSource {
}


// use super::data_item::DataItem;

// /// Pretend data source for testing multi-line scenarios.
// pub struct WSDataFrame {
//     current_byte_ix: usize,
//     num_bytes: usize,
// }

// impl WSDataFrame {
//     pub fn from(base64: String) -> WSDataFrame {
//         // Pretend to calc number of bytes
//         let num_bytes = 6;

//         // Data will be simulated in iterator

//         WSDataFrame {
//             current_byte_ix: 0,
//             num_bytes,
//         }
//     }

//     fn format_byte_one(self: &WSDataFrame) -> DataItem {
//         DataItem::from(vec![
//             "1|0|0|0|0 0 0 1",
//             "F|R|R|R| text  ",
//             "I|S|S|S|op code",
//             "N|V|V|V| (4 b) ",
//             " |1|2|3|       ",
//         ])
        
//     }

//     fn format_byte_two(self: &WSDataFrame) -> DataItem {
//         DataItem::from(vec![
//             "0|0 0 0 0 1 0 0",
//             "M|   4 bytes   ",
//             "A|(Payload len)",
//             "S|  (7 bits)   ",
//             "K|             ",
//         ])
//     }

//     fn format_payload_byte(self: &WSDataFrame, byte_index: usize) -> DataItem {
//         if byte_index < 5 {

        
//         DataItem::from(vec![
//             "1 0 0 1 0 0 0 1",
//             "               ",
//             &format!(" (Payload pt {})", byte_index + 1),
//             "    (8 bits)   ",
//         ])
//         }
//         else {
//             DataItem::from(vec![
//                 "1 0 0 1 0 0 0 1",
//                 "               ",
//                 &format!(" (Payload part the {})", byte_index + 1),
//                 "    (8 bits)   ",
//                 "  ++",
//             ])  
//         }
//     }
// }

// impl Iterator for WSDataFrame {
//     type Item = DataItem;

//     fn next(&mut self) -> Option<DataItem> {
//         if self.current_byte_ix < self.num_bytes {
//             // Get the next byte of data
//             let item = match self.current_byte_ix {
//                 0 => self.format_byte_one(),
//                 1 => self.format_byte_two(),
//                 _ => self.format_payload_byte(self.current_byte_ix),
//             };

//             // Advance the byte index
//             self.current_byte_ix += 1;

//             Some(item)
//         } else {
//             // No more data items to process
//             None
//         }
//     }
// }
