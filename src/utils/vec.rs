use std::cmp;

pub fn slice_copy<T: Clone>(replaced: &mut [T], src: &[T]) {
    let min_len = cmp::min(replaced.len(), src.len());

    for i in 0..min_len {
        replaced[i] = src[i].clone()
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::io::{BufReader, Read};

    #[test]
    fn c() -> io::Result<()> {
        let s = "1234567890ab".as_bytes();
        let mut reader = BufReader::new(s);

        let mut buf = vec![0_u8; 4];
        println!("len:{} cap:{}", buf.len(), buf.capacity());
        reader.read_exact(&mut buf)?;
        println!("{:?}", buf);

        buf.resize(8, 0);
        println!("len:{} cap:{}", buf.len(), buf.capacity());
        reader.read_exact(&mut buf)?;
        println!("{:?}", buf);

        Ok(())
    }

    #[test]
    fn b() -> io::Result<()> {
        let s = "234567890qwertyuiopasdfghjkl;xcvbnm,.234567890-wertyuifghjk`".as_bytes();
        let mut r = BufReader::new(s);
        _a(&mut r)?;

        let mut buf = vec![0_u8; 4];
        let size = io::copy(&mut r, &mut buf)?;
        println!("read size: {}, {:?}", size, buf);

        Ok(())
    }

    #[test]
    fn a() -> io::Result<()> {
        let s = "234567890qwertyuiopasdfghjkl;xcvbnm,.234567890-wertyuifghjk`"
            .as_bytes()
            .to_vec();
        let mut r = BufReader::new(&*s);
        _a(&mut r)?;

        let mut buf = vec![0_u8; 100];
        let size = r.read(&mut buf)?;
        println!("read size: {}, {:?}", size, buf);

        Ok(())
    }

    fn _a<R: ?Sized>(reader: &mut R) -> io::Result<()>
    where
        R: io::Read,
    {
        let mut data: Vec<u8> = vec![];
        let mut buf = vec![0_u8; 4];
        println!("len:{} cap:{}", buf.len(), buf.capacity());
        reader.read_exact(&mut buf)?;

        data.extend(&buf);
        println!("{:?}", data);
        buf.resize(2, 0);
        println!("len:{} cap:{}", buf.len(), buf.capacity());

        reader.read_exact(&mut buf)?;
        data.extend(&buf);
        println!("{:?}", data);

        buf.resize(3, 0);
        let szie = reader.read(&mut buf)?;
        println!("size:{}, {:?}", szie, &buf);

        Ok(())
    }
}
