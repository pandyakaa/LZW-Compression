# LZW - Compression

## Latar Belakang
Pengembangan aplikasi yang berkelanjutan (*sustainable*) menjadi sebuah kejaran yang semakin dicari pada akhir-akhir ini. Penggunaan sumberdaya diatur sedemikian rupa agar dapat terbentuk aplikasi yang sehemat mungkin, termasuk penggunaan energi. Terdapat berbagai metode untuk meningkatkan efisiensi sebuah aplikasi, salah satunya adalah mengurangi penggunaan *storage*. Metode yang aktif dikembangkan dan merupakan salah satu yang paling umum untuk diimplementasikan adalah kompresi.

Algoritma kompresi LZW (Lempel–Ziv–Welch) adalah sebuah metode kompresi *lossless* yang dibuat oleh Abraham Lempel, Jacob Ziv, and Terry Welch pada tahun 1970-1980-an. Konsep algoritma kompresi ini adalah mengurangi ukuran file (byte) dengan menggunakan *table-based lookup algorithm*. Algoritma ini merupakan pengembangan dari algoritma kompresi LZV yang biasa digunakan untuk file dengan format GIF, TIFF. Kompresi LZW juga dapat digunakan untuk kompresi file teks.

## Cara Penggunaan
1. Silahkan clone repository ini
2. Masukkan _file_ yang ingin dikompresi dan atau didekompresi pada **direktori** yang sama dengan _program_ ini, dan pastikan bahwa _file_ tersebut memiliki ekstensi **txt** atau **gif** atau **tif** atau **tiff**
3. Untuk menggunakan LZW Compression dengan file python => masuk ke folder pythonlzw :
Ketikkan perintah :
```
python lzw.py [compress | decompress] <namafile>.<ekstensifile>
```
4. Untuk menggunakan LZW Compression dengan file rust => masuk ke folder rustlzw :
Ketikkan perintah :
```
cargo run [compress | decompress] <namafile>.<ekstensifile>
```
5. Format _file_ hasil kompresi adalah :
```
<namafile>.txt
```
6. Format _file_ hasil dekompresi adalah :
```
<namafile>.<ekstensi>
```

## Keterangan 
1. Contoh _file_ dan hasil kompresinya ada pada folder example-file
2. Untuk _file_ yang ingin didekompresi, pastikan bahwa isi dari _file_ tersebut sudah **benar**(merupakan hasil dari kompresi suatu _file_)

## Asumsi
Penggunaan _character_ di dalam _file_ hanya terdiri dari :
1. **ASCII letters** : a,b,c,..,z dan A,B,C,..,Z
2. **Digits** : 0,1,..,9
3. **Punctuation**
4. **White Space**

## Referensi Pengerjaan
1. https://www.programcreek.com/2013/09/convert-image-to-string-in-python/
2. http://rosettacode.org/wiki/LZW_compression
3. https://www.geeksforgeeks.org/lzw-lempel-ziv-welch-compression-technique/
4. https://docs.python.org/2/library/string.html
5. https://stackoverflow.com/questions/646286/python-pil-how-to-write-png-image-to-string
