use arrayref::array_ref;
use ndarray::Array2;

pub struct Image {
    pub data: Array2<u8>,
    pub cmap: Vec<(u8, u8, u8)>,
}

impl Image {
    pub fn parse(data: &[u8]) -> Image {
        assert_eq!(&data[..4], b"FORM");
        let total_len = u32::from_be_bytes(*array_ref![data, 4, 4]) as usize;
        let data = &data[8..8 + total_len];
        let is_ilbm = match &data[..4] {
            b"PBM " => false,
            b"ILBM" => true,
            unk => panic!("unknown IFF format {unk:?}"),
        };
        let mut pos = 4;
        let mut image = None;
        let mut cmap = None;
        while pos != total_len {
            let chunk_hdr = array_ref![data, pos, 4];
            let chunk_len = u32::from_be_bytes(*array_ref![data, pos + 4, 4]) as usize;
            let chunk_data = &data[pos + 8..pos + chunk_len + 8];
            match chunk_hdr {
                b"BMHD" => {
                    assert_eq!(chunk_len, 0x14);
                    let width = u16::from_be_bytes(*array_ref![chunk_data, 0, 2]) as usize;
                    let height = u16::from_be_bytes(*array_ref![chunk_data, 2, 2]) as usize;
                    assert!(image.is_none());
                    image = Some(Array2::zeros((width, height)));
                }
                b"CMAP" => {
                    if is_ilbm {
                        assert_eq!(chunk_len, 0x30);
                    } else {
                        assert_eq!(chunk_len, 0x300);
                    }
                    let mut c = vec![];
                    for i in 0..chunk_len / 3 {
                        c.push((
                            chunk_data[i * 3],
                            chunk_data[i * 3 + 1],
                            chunk_data[i * 3 + 2],
                        ));
                    }
                    cmap = Some(c);
                }
                b"BODY" => {
                    let image = image.as_mut().unwrap();
                    let mut chunk_pos = 0;
                    for y in 0..image.dim().1 {
                        if is_ilbm {
                            for plane in 0..4 {
                                let lw = (image.dim().0 + 15) / 16 * 2;
                                let mut line = vec![0; lw];
                                let mut lpos = 0;
                                while lpos != lw {
                                    let b = chunk_data[chunk_pos];
                                    chunk_pos += 1;
                                    if b < 0x80 {
                                        let n = (b as usize) + 1;
                                        for _ in 0..n {
                                            line[lpos] = chunk_data[chunk_pos];
                                            lpos += 1;
                                            chunk_pos += 1;
                                        }
                                    } else {
                                        assert_ne!(b, 0x80);
                                        let n = 0x101 - (b as usize);
                                        let b = chunk_data[chunk_pos];
                                        chunk_pos += 1;
                                        for _ in 0..n {
                                            line[lpos] = b;
                                            lpos += 1;
                                        }
                                    }
                                }
                                for x in 0..image.dim().0 {
                                    image[(x, y)] |= (line[x / 8] >> (7 - (x & 7)) & 1) << plane;
                                }
                            }
                        } else {
                            let mut x = 0;
                            while x != image.dim().0 {
                                let b = chunk_data[chunk_pos];
                                chunk_pos += 1;
                                if b < 0x80 {
                                    let n = (b as usize) + 1;
                                    for _ in 0..n {
                                        image[(x, y)] = chunk_data[chunk_pos];
                                        x += 1;
                                        chunk_pos += 1;
                                    }
                                } else {
                                    assert_ne!(b, 0x80);
                                    let n = 0x101 - (b as usize);
                                    let b = chunk_data[chunk_pos];
                                    chunk_pos += 1;
                                    for _ in 0..n {
                                        image[(x, y)] = b;
                                        x += 1;
                                    }
                                }
                            }
                        }
                    }
                    assert_eq!(chunk_pos, chunk_data.len());
                }
                _ => {}
            }
            pos += chunk_len + 8;
            if pos & 1 != 0 {
                pos += 1;
            }
        }
        Image {
            data: image.unwrap(),
            cmap: cmap.unwrap(),
        }
    }

    pub fn intro_pal_fixup(&mut self) {
        for i in 0..0x20 {
            let (r, g, b) = self.cmap[i];
            self.cmap[i + 0x20] = (r / 2, g / 2, b / 2);
        }
    }
}
