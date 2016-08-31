// requires nightly rust
#![feature(test)]
#![feature(type_ascription)]

extern crate croaring;
extern crate roaring;
extern crate test;




#[cfg(test)]
mod tests {
    use test::Bencher;
    use test::black_box;
    use croaring::Bitmap;
    use roaring::RoaringBitmap;


    #[bench]
    fn croaring_create_bench(b: &mut Bencher) {
      b.iter(|| {
          let mut bitmap = Bitmap::create();
          bitmap.add(1);
          bitmap.add(2);
          bitmap.add(3);
          bitmap.add(1000);
          bitmap.add(10000);
          for i in 3000..3005u32 {
            bitmap.add(i);
          }
          black_box(bitmap);
      });
    }

    #[bench]
    fn roaring_create_bench(b: &mut Bencher) {
      b.iter(|| {
          // use `test::black_box` to prevent compiler optimizations from disregarding
          // unused values
          let  mut bitmap: RoaringBitmap<u32> = RoaringBitmap::new();
          bitmap.insert(1);
          bitmap.insert(2);
          bitmap.insert(3);
          bitmap.insert(1000);
          bitmap.insert(10000);
          for i in 3000..3005u32 {
            bitmap.insert(i);
          }
          black_box(bitmap);
      });
    }

}
