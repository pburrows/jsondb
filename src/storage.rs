use std::fs::File;
use std::path::Path;
use positioned_io::{WriteAt, RandomAccessFile, ReadAt};
use serde::{Serialize, Deserialize};
use serde_big_array::BigArray;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct FileHeader {
    type_id: [u8;10], // = [74 115 111 110 68 98 32 118 48 49];
    page_size: u16,
    #[serde(with = "BigArray")]
    future_space: [u8;500],
}

fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

pub fn create_file(path: &Path) {
    let file = File::create(path.as_os_str())
        .expect("Couldn't create file.");
    let file = RandomAccessFile::try_new(file).unwrap();
    let header = FileHeader {
        type_id: [74, 115, 111, 110, 68, 98, 32, 118, 48, 49], // JsonDb v01
        page_size: 512,
        future_space: [0;500]
    };
    let bytes: [u8;512] = vec_to_array(bincode::serialize(&header).unwrap());
    (&file).write_at(0, &bytes).unwrap();
}

pub fn read_header(path: &Path, page: &u32) {
    let bytes: Vec<u8> = read_page(path, page, 512);
    let mut header: Option<FileHeader> = None;
    let c_buf = bytes.as_ptr();
    let s = c_buf as *mut FileHeader;
    unsafe {
        let ref s2 = *s;
        header = Some(s2);
    }
    return header.unwrap();
}

pub fn read_page(path: &Path, page: &u32, page_size: &u16) {
    let file = RandomAccessFile::open(path).unwrap();
    let mut buf = vec![0;*page_size as usize]; //u32::from(&page_size)];
    let start_read: u64 = (page * (*page_size as u32)) as u64;
    let bytes_read = (&file).read_at(start_read, &mut buf).unwrap();
    return buf;
}

// #[cfg(test)]
// mod tests {
//     use crate::storage::struct_to_bytes;
//
//     #[test]
//     fn struct_to_bytes_is_compact() {
//        struct TestStruct {
//            type_id: [u8;10]
//        }
//         let item = TestStruct {
//             type_id: [74, 115, 111, 110, 68, 98, 32, 118, 48, 49], // JsonDb v01
//         };
//         let bytes: &[u8] = unsafe {struct_to_bytes(&item)};
//         assert_eq!(bytes[0], item.type_id[0]);
//         assert_eq!(bytes[1], item.type_id[1]);
//         assert_eq!(bytes[2], item.type_id[2]);
//         assert_eq!(bytes[3], item.type_id[3]);
//     }
// }